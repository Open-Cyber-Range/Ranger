import type React from 'react';
import {useEffect, useState} from 'react';
import type {Exercise} from 'src/models/exercise';
import type {EmailForm} from 'src/models/email';
import {
  Button,
  FormGroup,
  HTMLSelect,
  InputGroup,
  Intent,
  Label,
  TagInput,
} from '@blueprintjs/core';
import {useTranslation} from 'react-i18next';
import {Controller, type SubmitHandler, useForm} from 'react-hook-form';
import {
  useAdminGetDeploymentsQuery,
  useAdminGetEmailFormQuery,
  useAdminSendEmailMutation,
} from 'src/slices/apiSlice';
import {toastSuccess, toastWarning} from 'src/components/Toaster';
import Editor from '@monaco-editor/react';
import {type editor} from 'monaco-editor';
import useExerciseStreaming from 'src/hooks/useExerciseStreaming';
import {type Deployment} from 'src/models/deployment';
import useGetDeploymentUsers from 'src/hooks/useGetDeploymentUsers';
import {
  prepareEmailForDeploymentUser,
  preventDefaultOnEnter,
  prepareEmail,
  removeUnnecessaryEmailAddresses,
  validateEmailForm,
} from 'src/utils/email';
import {useEmailVariablesInEditor} from 'src/hooks/useEmailVariablesInEditor';
import {Tooltip2} from '@blueprintjs/popover2';
import EmailVariablesPopover from './EmailVariablesPopover';
import EmailVariablesInfo from './EmailVariablesInfo';

const SendEmail = ({exercise}: {exercise: Exercise}) => {
  const {t} = useTranslation();
  const {data: deployments} = useAdminGetDeploymentsQuery(exercise.id);
  const {data: fromAddress} = useAdminGetEmailFormQuery(exercise.id);
  const [sendMail, {isSuccess, error}] = useAdminSendEmailMutation();
  const [selectedDeployment, setSelectedDeployment] = useState<string | undefined>(undefined);
  const {deploymentUsers, fetchDeploymentUsers} = useGetDeploymentUsers();
  const [editorInstance, setEditorInstance]
  = useState<editor.IStandaloneCodeEditor | undefined>(undefined);
  const {emailVariables, insertVariable}
  = useEmailVariablesInEditor(selectedDeployment, editorInstance);
  const [isFetchingUsers, setIsFetchingUsers] = useState(false);
  useExerciseStreaming(exercise.id);

  const handleDeploymentChange = async (event: React.ChangeEvent<HTMLSelectElement>) => {
    const selectedValue = event.target.value;
    setSelectedDeployment(selectedValue);

    if (selectedValue === '') {
      return;
    }

    setIsFetchingUsers(true);

    try {
      if (selectedValue === 'wholeExercise') {
        if (!deployments || deployments.length === 0) {
          toastWarning(t('emails.noDeployments'));
          return;
        }

        const deploymentPromises = deployments.map(async deployment =>
          fetchDeploymentUsers(deployment.id, deployment.groupName));
        await Promise.all(deploymentPromises);
      } else {
        const selectedDeployment = deployments?.find(d => d.id === selectedValue);
        if (selectedDeployment) {
          await fetchDeploymentUsers(selectedDeployment.id, selectedDeployment.groupName);
        }
      }
    } catch {
      toastWarning(t('emails.fetchingUsersFail'));
    } finally {
      setIsFetchingUsers(false);
    }
  };

  const processWholeExercise = async (email: EmailForm) => {
    if (!deployments || deployments.length === 0) {
      toastWarning(t('emails.noDeployments'));
      return;
    }

    if (!deploymentUsers) {
      toastWarning(t('emails.fetchingUsersFail'));
      return;
    }

    const allEmailPromises = [];

    if (email.toAddresses.length > 0) {
      allEmailPromises.push(sendMail({
        email: prepareEmail(email, exercise.name),
        exerciseId: exercise.id,
      }));
    }

    removeUnnecessaryEmailAddresses(email);

    for (const deployment of deployments) {
      const currentDeploymentUsers = deploymentUsers[deployment.id];
      if (!currentDeploymentUsers || currentDeploymentUsers.length === 0) {
        continue;
      }

      const emailPromises = currentDeploymentUsers.map(async user =>
        sendMail({
          email: prepareEmailForDeploymentUser(
            email,
            exercise.name,
            deployment.name,
            user),
          exerciseId: exercise.id,
        }),
      );
      allEmailPromises.push(...emailPromises);
    }

    if (allEmailPromises.length === 0) {
      toastWarning(t('emails.creatingEmailsFail'));
      return;
    }

    await Promise.all(allEmailPromises);
  };

  const processSelectedDeployment = async (email: EmailForm, selectedDeploymentId: string) => {
    const currentDeploymentUsers = deploymentUsers?.[selectedDeploymentId];
    if (currentDeploymentUsers.length === 0) {
      toastWarning(t('emails.fetchingUsersFail'));
      return;
    }

    const deployment = deployments?.find(d => d.id === selectedDeployment);
    if (!deployment) {
      toastWarning(t('emails.noDeployment'));
      return;
    }

    const emailPromises = [];

    if (email.toAddresses.length > 0) {
      emailPromises.push(sendMail({
        email: prepareEmail(email, exercise.name),
        exerciseId: exercise.id,
      }));
    }

    removeUnnecessaryEmailAddresses(email);

    emailPromises.push(currentDeploymentUsers.map(async user =>
      sendMail({
        email: prepareEmailForDeploymentUser(
          email,
          exercise.name,
          deployment.name,
          user), exerciseId: exercise.id,
      }),
    ));

    await Promise.all(emailPromises);
  };

  const {handleSubmit, control} = useForm<EmailForm>({
    defaultValues: {
      toAddresses: [],
      replyToAddresses: [],
      ccAddresses: [],
      bccAddresses: [],
      subject: '',
      body: '',
    },
  });

  const onSubmit: SubmitHandler<EmailForm> = async email => {
    const {invalidEmailAddresses, isValid} = validateEmailForm(email);

    if (!isValid) {
      toastWarning(t('emails.invalidEmailAddress', {
        invalidEmailAddresses: invalidEmailAddresses.join(', '),
      }));
      return;
    }

    if (selectedDeployment === undefined || selectedDeployment === '') {
      await sendMail({email: prepareEmail(email, exercise.name), exerciseId: exercise.id});
    } else if (selectedDeployment === 'wholeExercise') {
      await processWholeExercise(email);
    } else if (selectedDeployment) {
      await processSelectedDeployment(email, selectedDeployment);
    }
  };

  useEffect(() => {
    if (isSuccess) {
      toastSuccess(t('emails.sendingSuccess'));
    }
  }, [isSuccess, t]);

  useEffect(() => {
    if (error) {
      if ('data' in error) {
        toastWarning(t('emails.sendingFail', {
          errorMessage: JSON.stringify(error.data),
        }));
      } else {
        toastWarning(t('emails.sendingFailWithoutMessage'));
      }
    }
  }, [error, t]);

  return (
    <form onSubmit={handleSubmit(onSubmit)} onKeyDown={preventDefaultOnEnter}>
      <div>
        <FormGroup
          label={t('emails.form.from.title')}
        >
          <InputGroup
            large
            disabled
            placeholder={fromAddress ?? ''}
          />
        </FormGroup>
        <FormGroup label={t('emails.form.deploymentSelector.title')}>
          <HTMLSelect
            autoFocus
            large
            fill
            value={selectedDeployment ?? ''}
            onChange={handleDeploymentChange}
          >
            <option value=''>
              {t('emails.form.deploymentSelector.placeholder')}
            </option>
            <option value='wholeExercise'>
              {t('emails.form.deploymentSelector.wholeExercise')}
            </option>
            {deployments?.map((deployment: Deployment) => (
              <option key={deployment.id} value={deployment.id}>
                {deployment.name}
              </option>
            ))}
          </HTMLSelect>
        </FormGroup>
        <Controller
          control={control}
          name='toAddresses'
          rules={{
            required: selectedDeployment ? false : (t('emails.form.to.required') ?? ''),
          }}
          render={({
            field: {onChange, ref, value}, fieldState: {error},
          }) => {
            const intent = error ? Intent.DANGER : Intent.NONE;
            return (
              <FormGroup
                labelInfo={selectedDeployment ? '' : '(required)'}
                helperText={error?.message}
                intent={intent}
                label={t('emails.form.to.title')}
              >
                <TagInput
                  large
                  addOnBlur
                  addOnPaste
                  inputRef={ref}
                  intent={intent}
                  placeholder={t('emails.form.emailPlaceholder') ?? ''}
                  values={value}
                  tagProps={{interactive: true}}
                  onChange={(values: React.ReactNode[]) => {
                    onChange(values.filter(Boolean).map(String));
                  }}
                />
              </FormGroup>
            );
          }}
        />
        <Controller
          control={control}
          name='replyToAddresses'
          render={({
            field: {onChange, ref, value},
          }) => (
            <FormGroup
              label={t('emails.form.replyTo.title')}
            >
              <TagInput
                large
                addOnBlur
                addOnPaste
                inputRef={ref}
                placeholder={t('emails.form.emailPlaceholder') ?? ''}
                values={value}
                tagProps={{interactive: true}}
                onChange={(values: React.ReactNode[]) => {
                  onChange(values.filter(Boolean).map(String));
                }}
              />
            </FormGroup>
          )}
        />
        <Controller
          control={control}
          name='ccAddresses'
          render={({
            field: {onChange, ref, value},
          }) => (
            <FormGroup
              label={t('emails.form.cc.title')}
            >
              <TagInput
                large
                addOnBlur
                addOnPaste
                inputRef={ref}
                placeholder={t('emails.form.emailPlaceholder') ?? ''}
                values={value}
                tagProps={{interactive: true}}
                onChange={(values: React.ReactNode[]) => {
                  onChange(values.filter(Boolean).map(String));
                }}
              />
            </FormGroup>
          )}
        />
        <Controller
          control={control}
          name='bccAddresses'
          render={({
            field: {onChange, ref, value},
          }) => (
            <FormGroup
              label={t('emails.form.bcc.title')}
            >
              <TagInput
                large
                addOnBlur
                addOnPaste
                inputRef={ref}
                placeholder={t('emails.form.emailPlaceholder') ?? ''}
                values={value}
                tagProps={{interactive: true}}
                onChange={(values: React.ReactNode[]) => {
                  onChange(values.filter(Boolean).map(String));
                }}
              />
            </FormGroup>
          )}
        />
        <Controller
          control={control}
          name='subject'
          rules={{required: t('emails.form.subject.required') ?? ''}}
          render={({
            field: {onChange, onBlur, ref, value}, fieldState: {error},
          }) => {
            const intent = error ? Intent.DANGER : Intent.NONE;
            return (
              <FormGroup
                helperText={error?.message}
                intent={intent}
                label={
                  <div className='flex justify-between items-end'>
                    <Label>
                      {t('emails.form.subject.title')}
                      <span className='bp4-text-muted'>{t('emails.form.required')}</span>
                    </Label>
                    <EmailVariablesInfo emailVariables={emailVariables}/>
                  </div>
                }
              >
                <InputGroup
                  large
                  intent={intent}
                  value={value}
                  inputRef={ref}
                  id='email-subject'
                  onChange={onChange}
                  onBlur={onBlur}
                />
              </FormGroup>
            );
          }}
        />
        <Controller
          control={control}
          name='body'
          rules={{required: t('emails.form.body.required') ?? ''}}
          render={({
            field: {onChange, value}, fieldState: {error},
          }) => {
            const intent = error ? Intent.DANGER : Intent.NONE;
            return (
              <FormGroup
                helperText={error?.message}
                intent={intent}
                label={
                  <div className='flex justify-between items-end'>
                    <Label>
                      {t('emails.form.body.title')}
                      <span className='bp4-text-muted'>{t('emails.form.required')}</span>
                    </Label>
                    <EmailVariablesPopover
                      emailVariables={emailVariables}
                      insertVariable={insertVariable}/>
                  </div>
                }
              >
                <div className='h-[40vh] p-[0.5vh] rounded-sm shadow-inner'>
                  <Editor
                    value={value}
                    defaultLanguage='html'
                    onChange={value => {
                      onChange(value ?? '');
                    }}
                    onMount={editor => {
                      setEditorInstance(editor);
                    }}
                  />
                </div>
              </FormGroup>
            );
          }}
        />
      </div>
      <Tooltip2
        content={t('emails.sendButtonDisabled') ?? ''}
        disabled={!isFetchingUsers}
      >
        <Button
          large
          type='submit'
          intent='primary'
          text={t('emails.send')}
          disabled={isFetchingUsers}
        />
      </Tooltip2>
    </form>
  );
};

export default SendEmail;
