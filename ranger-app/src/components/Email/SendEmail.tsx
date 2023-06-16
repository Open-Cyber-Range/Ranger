
import React, {useEffect} from 'react';
import type {EmailForm, Exercise} from 'src/models/exercise';
import {
  Button,
  FormGroup,
  InputGroup,
  Intent,
  TagInput,
} from '@blueprintjs/core';
import {useTranslation} from 'react-i18next';
import {Controller, type SubmitHandler, useForm} from 'react-hook-form';
import {
  useAdminGetEmailFormQuery,
  useAdminSendEmailMutation,
} from 'src/slices/apiSlice';
import {toastSuccess, toastWarning} from 'src/components/Toaster';
import nunjucks from 'nunjucks';
import Editor from '@monaco-editor/react';
import useExerciseStreaming from 'src/hooks/useExerciseStreaming';

const SendEmail = ({exercise}: {exercise: Exercise}) => {
  const {t} = useTranslation();
  const {data: fromAddress} = useAdminGetEmailFormQuery(exercise.id);
  const [sendMail, {isSuccess, error}] = useAdminSendEmailMutation();
  useExerciseStreaming(exercise.id);

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
    const invalidEmailAddresses = [validateEmails(email.toAddresses)].flat()
      .concat(validateEmails(email.replyToAddresses ?? []))
      .concat(validateEmails(email.ccAddresses ?? []))
      .concat(validateEmails(email.bccAddresses ?? []));

    if (invalidEmailAddresses.length > 0) {
      toastWarning(t('emails.invalidEmailAddress', {
        invalidEmailAddresses: invalidEmailAddresses.join(', '),
      }));
      return;
    }

    email.subject = nunjucks
      .renderString(
        email.subject,
        {exerciseName: exercise.name});

    email.body = nunjucks
      .renderString(
        email.body,
        {exerciseName: exercise.name});

    await sendMail({email, exerciseId: exercise.id});
  };

  const validateEmails = (emails: string[]) => {
    const faultyEmails = [];
    for (const email of emails) {
      if (email.trim().length > 0
       && !/^[\w.%+-]+@[a-z\d.-]+\.[a-z]{2,4}$/i.test(email)) {
        faultyEmails.push(email);
      }
    }

    if (faultyEmails.length > 0) {
      return faultyEmails;
    }

    return [];
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
    <form onSubmit={handleSubmit(onSubmit)}>
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
        <Controller
          control={control}
          name='toAddresses'
          rules={{required: t('emails.form.to.required') ?? ''}}
          render={({
            field: {onChange, ref, value}, fieldState: {error},
          }) => {
            const intent = error ? Intent.DANGER : Intent.NONE;
            return (
              <FormGroup
                labelInfo='(required)'
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
                  onChange={onChange}
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
                onChange={onChange}
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
                onChange={onChange}
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
                onChange={onChange}
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
                labelInfo='(required)'
                helperText={error?.message}
                intent={intent}
                label={t('emails.form.subject.title')}
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
                labelInfo='(required)'
                helperText={error?.message}
                intent={intent}
                label={t('emails.form.body.title')}
              >
                <div className='h-[40vh] p-[0.5vh] rounded-sm shadow-inner'>
                  <Editor
                    value={value}
                    defaultLanguage='html'
                    onChange={onChange}
                  />
                </div>
              </FormGroup>
            );
          }}
        />
      </div>
      <Button
        large
        type='submit'
        intent='primary'
        text={t('emails.send')}
      />
    </form>
  );
};

export default SendEmail;
