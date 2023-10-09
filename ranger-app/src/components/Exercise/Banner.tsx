import type React from 'react';
import {useTranslation} from 'react-i18next';
import type {Exercise, Banner} from 'src/models/exercise';
import {skipToken} from '@reduxjs/toolkit/query';
import {Button, FormGroup, InputGroup, Intent} from '@blueprintjs/core';
import {Controller, type SubmitHandler, useForm} from 'react-hook-form';
import {
  useAdminAddBannerMutation,
  useAdminDeleteBannerMutation,
  useAdminGetBannerQuery,
  useAdminUpdateBannerMutation,
} from 'src/slices/apiSlice';
import {useEffect} from 'react';

const BannerView = ({exercise}:
{
  exercise: Exercise;
}) => {
  const {t} = useTranslation();
  const {data: existingBanner} = useAdminGetBannerQuery(exercise?.id ?? skipToken);
  const [addBanner] = useAdminAddBannerMutation();
  const [updateBanner] = useAdminUpdateBannerMutation();
  const [deleteBanner] = useAdminDeleteBannerMutation();

  const {handleSubmit, control, reset, setValue} = useForm<Banner>();

  useEffect(() => {
    if (existingBanner) {
      setValue('name', existingBanner.name || '');
      setValue('content', existingBanner.content || '');
    }
  }, [existingBanner, setValue]);

  const onCreate: SubmitHandler<Banner> = async newBanner => {
    await addBanner({newBanner, exerciseId: exercise.id});
  };

  const onUpdate: SubmitHandler<Banner> = async updatedBanner => {
    await updateBanner({updatedBanner, exerciseId: exercise.id});
  };

  const onDelete: SubmitHandler<Banner> = async () => {
    await deleteBanner({exerciseId: exercise.id});
    reset();
  };

  return (
    <div>
      <form
        onSubmit={existingBanner ? handleSubmit(onUpdate) : handleSubmit(onCreate)}
        onReset={handleSubmit(onDelete)}
      >
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
          text={existingBanner ? t('update') : t('create')}
        />
        <Button
          large
          type='reset'
          intent='danger'
          text={t('delete')}
        />
      </form>
    </div>
  );
};

export default BannerView;
