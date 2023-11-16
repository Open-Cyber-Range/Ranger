import type React from 'react';
import {useEffect, useState} from 'react';
import type {Exercise} from 'src/models/exercise';
import type {
  EmailForm,
  EmailTemplate,
  EmailTemplateForm,
  NewEmailTemplate,
} from 'src/models/email';
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
  useAdminAddEmailTemplateMutation,
  useAdminGetDeploymentsQuery,
  useAdminGetEmailFormQuery,
  useAdminGetEmailTemplatesQuery,
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
  prepareForPreviewWithoutUserOrDeployment,
  prepareForPreview,
  openNewBlobWindow,
} from 'src/utils/email';
import {useEmailVariablesInEditor} from 'src/hooks/useEmailVariablesInEditor';
import {Tooltip2} from '@blueprintjs/popover2';
import {sortByProperty} from 'sort-by-property';
import EmailVariablesPopover from './EmailVariablesPopover';
import EmailVariablesInfo from './EmailVariablesInfo';
import TemplateSaveDialog from './TemplateSaveDialog';

const SendEmail = ({exercise}: {exercise: Exercise}) => {
  const {t} = useTranslation();
  const {data: potentialEmailTemplates, refetch: refetchEmailTemplates}
    = useAdminGetEmailTemplatesQuery();
  const {data: deployments} = useAdminGetDeploymentsQuery(exercise.id);
  const {data: fromAddress} = useAdminGetEmailFormQuery(exercise.id);
  const [sendMail, {isSuccess: sendSuccess, error: sendError}] = useAdminSendEmailMutation();
  const [addEmailTemplate, {isSuccess: emailTemplateSuccess, error: emailTemplateError}]
    = useAdminAddEmailTemplateMutation();
  const [selectedDeployment, setSelectedDeployment] = useState<string | undefined>(undefined);
  const [selectedEmailTemplate, setSelectedEmailTemplate] = useState<string | undefined>(undefined);
  const [emailTemplateContent, setEmailTemplateContent] = useState<string | undefined>(undefined);
  const [isAddEmailTemplateDialogOpen, setIsAddEmailTemplateDialogOpen] = useState(false);
  const {deploymentUsers, fetchDeploymentUsers} = useGetDeploymentUsers();
  const [editorInstance, setEditorInstance]
  = useState<editor.IStandaloneCodeEditor | undefined>(undefined);
  const {emailVariables, insertVariable}
  = useEmailVariablesInEditor(selectedDeployment, editorInstance);
  const [isFetchingUsers, setIsFetchingUsers] = useState(false);
  const [editorContent, setEditorContent] = useState('');
  useExerciseStreaming(exercise.id);

  let emailTemplates = potentialEmailTemplates ?? [];
  emailTemplates = emailTemplates.slice().sort(sortByProperty('name', 'desc'));

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

  const handleEmailTemplateChange = async (event: React.ChangeEvent<HTMLSelectElement>) => {
    const selectedTemplate = event.target.value;
    setSelectedEmailTemplate(selectedTemplate);
    if (selectedTemplate !== '' && emailTemplates) {
      const selectedEmailTemplate = emailTemplates.find(
        (template: EmailTemplate) => template.name === selectedTemplate,
      );
      if (selectedEmailTemplate) {
        setEmailTemplateContent(selectedEmailTemplate.content);
      }
    } else {
      setEmailTemplateContent('');
    }
  };

  const handleAddEmailTemplate = async (templateForm: EmailTemplateForm) => {
    if (editorInstance?.getValue() === '') {
      toastWarning(t('emails.template.required'));
      return;
    }

    const templateData: NewEmailTemplate = {
      name: templateForm.name,
      content: editorInstance?.getValue() ?? '',
    };
    setIsAddEmailTemplateDialogOpen(false);
    setSelectedEmailTemplate(templateData.name);
    setEmailTemplateContent(templateData.content);
    await addEmailTemplate(templateData);
    await refetchEmailTemplates();
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

  const {handleSubmit, control, watch} = useForm<EmailForm>({
    defaultValues: {
      toAddresses: [],
      replyToAddresses: [],
      ccAddresses: [],
      bccAddresses: [],
      subject: '',
      template: '',
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

  const emailSubject = watch('subject');

  const previewHtmlContent = () => {
    let preparedEmailSubject
     = prepareForPreviewWithoutUserOrDeployment(emailSubject, exercise.name);
    let preparedEditorContent
     = prepareForPreviewWithoutUserOrDeployment(editorContent, exercise.name);

    if (selectedDeployment === 'wholeExercise') {
      const deployment: Deployment | undefined = deployments?.[0];
      preparePreviews(deployment);
    } else if (selectedDeployment) {
      const deployment = deployments?.find(d => d.id === selectedDeployment);
      preparePreviews(deployment);
    }

    const combinedContent = `
      <h2>${preparedEmailSubject}</h2> 
      ${preparedEditorContent}
    `;

    openNewBlobWindow(combinedContent);

    function preparePreviews(deployment: Deployment | undefined) {
      if (deployment) {
        const user = deploymentUsers?.[deployment.id]?.[0];
        if (user) {
          preparedEmailSubject = prepareForPreview(
            emailSubject,
            exercise.name,
            deployment.name,
            user,
          );
          preparedEditorContent = prepareForPreview(
            editorContent,
            exercise.name,
            deployment.name,
            user,
          );
        }
      }
    }
  };

  useEffect(() => {
    if (sendSuccess) {
      toastSuccess(t('emails.sendingSuccess'));
    }

    if (emailTemplateSuccess) {
      toastSuccess(t('emails.addingTemplateSuccess'));
    }
  }, [emailTemplateSuccess, sendSuccess, t]);

  useEffect(() => {
    if (sendError) {
      if ('data' in sendError) {
        toastWarning(t('emails.sendingFail', {
          errorMessage: JSON.stringify(sendError.data),
        }));
      } else {
        toastWarning(t('emails.sendingFailWithoutMessage'));
      }
    }

    if (emailTemplateError) {
      if ('data' in emailTemplateError) {
        toastWarning(t('emails.addingTemplateFail', {
          errorMessage: JSON.stringify(emailTemplateError.data),
        }));
      } else {
        toastWarning(t('emails.addingTemplateFailWithoutMessage'));
      }
    }
  }, [emailTemplateError, sendError, t]);

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
        <FormGroup label={t('emails.form.template.title')}>
          <HTMLSelect
            autoFocus
            large
            fill
            value={selectedEmailTemplate ?? ''}
            onChange={handleEmailTemplateChange}
          >
            <option value=''>
              {t('emails.form.template.placeholder')}
            </option>
            {emailTemplates?.map((emailTemplate: EmailTemplate) => (
              <option key={emailTemplate.name} value={emailTemplate.name}>
                {emailTemplate.name}
              </option>
            ))}
          </HTMLSelect>
        </FormGroup>
        <Controller
          control={control}
          name='body'
          rules={{required: t('emails.form.body.required') ?? ''}}
          render={({
            field: {onChange}, fieldState: {error},
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
                    value={editorContent || emailTemplateContent}
                    defaultLanguage='html'
                    onChange={value => {
                      setEditorContent(value ?? '');
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
      <div className='flex justify-end mt-[1rem] mb-[1rem] gap-[0.5rem] '>
        <Button
          large
          outlined
          intent='primary'
          text={t('emails.form.preview')}
          type='button'
          onClick={previewHtmlContent}
        />
        <Button
          large
          intent='success'
          text={t('emails.form.template.save')}
          onClick={() => {
            setIsAddEmailTemplateDialogOpen(true);
          }}
        />
        <Tooltip2
          content={t('emails.form.sendButtonDisabled') ?? ''}
          disabled={!isFetchingUsers}
        >
          <Button
            large
            type='submit'
            intent='primary'
            text={t('emails.form.send')}
            disabled={isFetchingUsers}
          />
        </Tooltip2>
      </div>
      <TemplateSaveDialog
        isOpen={isAddEmailTemplateDialogOpen}
        title={t('emails.form.template.title')}
        onCancel={() => {
          setIsAddEmailTemplateDialogOpen(false);
        }}
        onSubmit={handleAddEmailTemplate}
      />
    </form>
  );
};

export default SendEmail;
