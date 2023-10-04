import type React from 'react';
import {useTranslation} from 'react-i18next';
import type {Exercise, Banner} from 'src/models/exercise';
import {skipToken} from '@reduxjs/toolkit/query';
import {Button, FormGroup, InputGroup, Intent} from '@blueprintjs/core';
import {Controller, type SubmitHandler, useForm} from 'react-hook-form';
import {
  useAdminAddBannerMutation,
  useAdminGetBannerQuery,
  useAdminUpdateBannerMutation,
} from 'src/slices/apiSlice';

const BannerView = ({exercise}:
											 {
												 exercise: Exercise;
											 }) => {
  const {t} = useTranslation();
  const {data: existingBanner} = useAdminGetBannerQuery(exercise?.id ?? skipToken);
  const [addBanner, {createSuccess, createError}] = useAdminAddBannerMutation();
  const [updateBanner, {updateSuccess, updateError}] = useAdminUpdateBannerMutation();

  const {handleSubmit, control} = useForm<Banner>({
    defaultValues: {
      name: '',
      content: '',
    },
  });

  const onCreate: SubmitHandler<Banner> = async newBanner => {
    await addBanner({newBanner, exerciseId: exercise.id});
  };

  const onUpdate: SubmitHandler<Banner> = async updatedBanner => {
    await updateBanner({updatedBanner, exerciseId: exercise.id});
  };

  function submitHandling(existingBanner: Banner | undefined) {
    if (existingBanner) {
      return (
        handleSubmit(onUpdate)
      );
    }

    return handleSubmit(onCreate);
  }

  return (
    <div>
      <form onSubmit={submitHandling(existingBanner)}>
        <Controller
          control={control}
          name='name'
          rules={{required: t('banners.required') ?? ''}}
          render={({
										 field: {onChange, onBlur, ref, value}, fieldState: {error},
									 }) => {
            const intent = error ? Intent.DANGER : Intent.NONE;
            return (
              <FormGroup
                helperText={error?.message}
                intent={intent}
                label={t('banners.name')}
              >
                <InputGroup
                  large
                  intent={intent}
                  value={value}
                  inputRef={ref}
                  id='banner-name'
                  onChange={onChange}
                  onBlur={onBlur}
                />
              </FormGroup>
            );
          }}
        />
        <Controller
          control={control}
          name='content'
          rules={{required: t('banners.required') ?? ''}}
          render={({
										 field: {onChange, onBlur, ref, value}, fieldState: {error},
									 }) => (
            <FormGroup
    helperText={error?.message}
    label={t('banners.content')}
  >
    <InputGroup
                large
                value={value}
                inputRef={ref}
                id='banner-content'
                onChange={onChange}
                onBlur={onBlur}
              />
  </FormGroup>
          )}
        />
        <Button
          large
          type='submit'
          intent='primary'
          text={t('create')}
        />
      </form>
    </div>
  );
};

export default BannerView;
