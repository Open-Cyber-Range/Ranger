import React, {useEffect} from 'react';
import type {EmailForm, Exercise} from 'src/models/exercise';
import {
  Button,
  FormGroup,
  InputGroup,
  Intent,
  TextArea,
} from '@blueprintjs/core';
import {useTranslation} from 'react-i18next';
import {Controller, type SubmitHandler, useForm} from 'react-hook-form';
import {useSendMailMutation} from 'src/slices/apiSlice';
import {toastSuccess, toastWarning} from 'src/components/Toaster';
import nunjucks from 'nunjucks';

const SendEmail = ({exercise}: {exercise: Exercise}) => {
  const {t} = useTranslation();
  const [sendMail, {isSuccess, error}] = useSendMailMutation();

  const {handleSubmit, control} = useForm<EmailForm>({
    defaultValues: {
      toAddress: '',
      subject: '',
      body: '',
    },
  });

  const onSubmit: SubmitHandler<EmailForm> = async email => {
    const reformattedBody = nunjucks
      .renderString(email.body, {exerciseName: exercise.name});
    email.body = reformattedBody;
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
    <form className='EmailForm' onSubmit={handleSubmit(onSubmit)}>
      <div>
        <Controller
          control={control}
          name='toAddress'
          rules={{required: t('emails.form.to.required') ?? ''}}
          render={({
            field: {onChange, onBlur, ref, value}, fieldState: {error},
          }) => {
            const intent = error ? Intent.DANGER : Intent.NONE;
            return (
              <FormGroup
                labelFor='email-to'
                labelInfo='(required)'
                helperText={error?.message}
                intent={intent}
                label={t('emails.form.to.title')}
              >
                <InputGroup
                  large
                  intent={intent}
                  value={value}
                  inputRef={ref}
                  id='email-to'
                  onChange={onChange}
                  onBlur={onBlur}
                />
              </FormGroup>
            );
          }}
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
            field: {onChange, onBlur, ref, value}, fieldState: {error},
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
                <TextArea
                  large
                  growVertically
                  fill
                  intent={intent}
                  value={value}
                  inputRef={ref}
                  id='email-body'
                  onChange={onChange}
                  onBlur={onBlur}
                />
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
