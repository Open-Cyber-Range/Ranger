import type React from 'react';
import {useTranslation} from 'react-i18next';
import type {Exercise, Banner, BannerVariable} from 'src/models/exercise';
import {skipToken} from '@reduxjs/toolkit/query';
import {Button, FormGroup, InputGroup, Intent, Label, Menu, MenuItem} from '@blueprintjs/core';
import {useNavigate} from 'react-router-dom';
import {Controller, type SubmitHandler, useForm} from 'react-hook-form';
import {
  useAdminAddBannerMutation,
  useAdminDeleteBannerMutation,
  useAdminGetBannerQuery, useAdminGetDeploymentsQuery,
  useAdminUpdateBannerMutation,
} from 'src/slices/apiSlice';
import {useEffect, useState} from 'react';
import {ActiveTab} from 'src/models/exercise';
import {toastSuccess, toastWarning} from 'src/components/Toaster';
import Editor from "@monaco-editor/react";
import {editor} from "monaco-editor";
import {useBannerVariablesInEditor} from "../../hooks/useBannerVariablesInEditor";
import {Popover2} from "@blueprintjs/popover2";
import useGetDeploymentUsers from "../../hooks/useGetDeploymentUsers";
import {prepareBannerForDeploymentUser} from "../../utils/banner";

type BannerVariablesProps = {
  bannerVariables: BannerVariable[];
  insertVariable: (variableName: string) => void;
};

const BannerVariablesMenu = ({bannerVariables, insertVariable}: BannerVariablesProps) => (
  <Menu>
    {bannerVariables.map(variable => (
      <MenuItem
        key={variable.name}
        text={variable.content}
        onClick={() => {
          insertVariable(variable.name);
        }}
      />
    ))}
  </Menu>
);

const BannerVariablesPopover = ({bannerVariables, insertVariable}: BannerVariablesProps) => {
  const {t} = useTranslation();

  return (
    <Popover2
      content={<BannerVariablesMenu
        bannerVariables={bannerVariables}
        insertVariable={insertVariable}/>}
      position='bottom-left'
    >
      <Button minimal icon='insert' text={t('emails.variables.insert')}/>
    </Popover2>
  );
};

const BannerView = ({exercise}: {exercise: Exercise}) => {
  const {t} = useTranslation();
  const navigate = useNavigate();
  const {data: deployments} = useAdminGetDeploymentsQuery(exercise.id);
  const {deploymentUsers, fetchDeploymentUsers} = useGetDeploymentUsers();
  let {data: existingBanner} = useAdminGetBannerQuery(exercise?.id ?? skipToken);

  const [selectedDeployment, setSelectedDeployment]
    = useState<string | undefined>(undefined);
  const [editorInstance, setEditorInstance]
    = useState<editor.IStandaloneCodeEditor | undefined>(undefined);
  const {bannerVariables, insertVariable}
    = useBannerVariablesInEditor(selectedDeployment, editorInstance);
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

  const processWholeExercise = async (banner: Banner) => {
    if (!deployments || deployments.length === 0) {
      toastWarning(t('emails.noDeployments'));
      return;
    }

    if (!deploymentUsers) {
      toastWarning(t('emails.fetchingUsersFail'));
      return;
    }

    for (const deployment of deployments) {
      console.log(prepareBannerForDeploymentUser(
        banner,
        exercise.name,
        exercise.name)
      );
    }
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
    await processWholeExercise(newBanner);
    await addBanner({newBanner, exerciseId: exercise.id});
  };

  const onUpdate: SubmitHandler<Banner> = async updatedBanner => {
    await processWholeExercise(updatedBanner);
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
              label={
                <div className='flex justify-between items-end'>
                  <Label>
                    {t('banners.content')}
                    <span className='bp4-text-muted'>{t('banners.required')}</span>
                  </Label>
                  <BannerVariablesPopover
                    bannerVariables={bannerVariables}
                    insertVariable={insertVariable}/>
                </div>
              }
            >
              <div className='h-[40vh] p-[0.5vh] rounded-sm shadow-inner'>
                <Editor
                  value={value}
                  onChange={value => {
                    onChange(value ?? '');
                  }}
                  onMount={editor => {
                    setEditorInstance(editor);
                  }}
                />
              </div>
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
