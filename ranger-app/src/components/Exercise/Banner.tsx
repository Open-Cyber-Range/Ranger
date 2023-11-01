import type React from 'react';
import {useTranslation} from 'react-i18next';
import type {Exercise, Banner} from 'src/models/exercise';
import {skipToken} from '@reduxjs/toolkit/query';
import {Button, FormGroup, InputGroup, Intent} from '@blueprintjs/core';
import {useNavigate} from 'react-router-dom';
import {Controller, type SubmitHandler, useForm} from 'react-hook-form';
import {
  useAdminAddBannerMutation,
  useAdminDeleteBannerMutation,
  useAdminGetBannerQuery,
  useAdminUpdateBannerMutation,
} from 'src/slices/apiSlice';
import {useEffect, useState} from 'react';
import {ActiveTab} from 'src/models/exercise';
import {toastSuccess, toastWarning} from 'src/components/Toaster';

const BannerView = ({exercise}: {exercise: Exercise}) => {
  const {t} = useTranslation();
  const navigate = useNavigate();
  let {data: existingBanner} = useAdminGetBannerQuery(exercise?.id ?? skipToken);
  const [, setActiveTab] = useState<ActiveTab>(ActiveTab.Banner);

  const [addBanner, {isSuccess: isAddSuccess, error: addError}] = useAdminAddBannerMutation();
  const [updateBanner, {isSuccess: isUpdateSuccess, error: updateError}]
    = useAdminUpdateBannerMutation();
  const [deleteBanner, {isSuccess: isDeleteSuccess, error: deleteError}]
    = useAdminDeleteBannerMutation();

  const {handleSubmit, control, setValue} = useForm<Banner>();
  const routeChange = () => {
    navigate('');
    setActiveTab(ActiveTab.Dash);
  };

  useEffect(() => {
    if (isAddSuccess) {
      toastSuccess(t('banners.createSuccess'));
    } else if (isUpdateSuccess) {
      toastSuccess(t('banners.updateSuccess'));
    } else if (isDeleteSuccess) {
      toastSuccess(t('banners.deleteSuccess'));
    }
  }, [isAddSuccess, isUpdateSuccess, isDeleteSuccess, t]);

  useEffect(() => {
    if (addError) {
      if ('data' in addError) {
        toastWarning(t('banners.createFail', {
          errorMessage: JSON.stringify(addError.data),
        }));
      } else {
        toastWarning(t('banners.createFailWithoutMessage'));
      }
    } else if (updateError) {
      if ('data' in updateError) {
        toastWarning(t('banners.updateFail', {
          errorMessage: JSON.stringify(updateError.data),
        }));
      } else {
        toastWarning(t('banners.updateFailWithoutMessage'));
      }
    } else if (deleteError) {
      if ('data' in deleteError) {
        toastWarning(t('banners.deleteFail', {
          errorMessage: JSON.stringify(deleteError.data),
        }));
      } else {
        toastWarning(t('banners.deleteFailWithoutMessage'));
      }
    }
  }, [addError, updateError, deleteError, t]);

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
    setValue('name', '');
    setValue('content', '');
    existingBanner = undefined;
  };

  return (
    <div>
      <form
        onSubmit={existingBanner ? handleSubmit(onUpdate) : handleSubmit(onCreate)}
        onReset={handleSubmit(onDelete)}
        onClick={routeChange}
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
