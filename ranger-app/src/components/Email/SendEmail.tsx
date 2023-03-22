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
import {useSendMailMutation} from 'src/slices/apiSlice';
import {toastSuccess, toastWarning} from 'src/components/Toaster';
import nunjucks from 'nunjucks';
import Editor from '@monaco-editor/react';
import styled from 'styled-components';

const EditorHolder = styled.div`
  height: 40vh;
`;

const SendEmail = ({exercise}: {exercise: Exercise}) => {
  const {t} = useTranslation();
  const [sendMail, {isSuccess, error}] = useSendMailMutation();

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
    email.subject = nunjucks
      .renderString(
        email.subject,
        // Needs more variables for participant info, entity info, etc.
        {exerciseName: exercise.name});

    email.body = nunjucks
      .renderString(
        email.body,
        // Needs more variables for participant info, entity info, etc.
        {exerciseName: exercise.name});

    await sendMail({email, exerciseId: exercise.id});
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
              labelFor='email-reply-to'
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
              labelFor='email-cc'
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
              labelFor='email-bcc'
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
                labelFor='email-subject'
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
                labelFor='email-body'
                labelInfo='(required)'
                helperText={error?.message}
                intent={intent}
                label={t('emails.form.body.title')}
              >
                <EditorHolder>
                  <Editor
                    value={value}
                    defaultLanguage='html'
                    onChange={onChange}
                  />
                </EditorHolder>
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
