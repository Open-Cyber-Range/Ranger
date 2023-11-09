import type React from 'react';
import {useEffect, useState} from 'react';
import {useTranslation} from 'react-i18next';
import type {Banner, BannerVariable, Exercise} from 'src/models/exercise';
import {skipToken} from '@reduxjs/toolkit/query';
import {
  Button,
  FormGroup,
  InputGroup,
  Intent,
  Label,
  Menu,
  MenuItem,
} from '@blueprintjs/core';
import {Controller, type SubmitHandler, useForm} from 'react-hook-form';
import {
  useAdminAddBannerMutation,
  useAdminDeleteBannerMutation,
  useAdminGetBannerQuery,
  useAdminUpdateBannerMutation,
} from 'src/slices/apiSlice';
import {toastSuccess, toastWarning} from 'src/components/Toaster';
import Editor from '@monaco-editor/react';
import {type editor} from 'monaco-editor';
import {Popover2} from '@blueprintjs/popover2';
import {
  useBannerVariablesInEditor,
} from 'src/hooks/useBannerVariablesInEditor';

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
  const {data} = useAdminGetBannerQuery(exercise?.id ?? skipToken);
  const [addBanner, {error: addError}] = useAdminAddBannerMutation();
  const [updateBanner, {error: updateError}] = useAdminUpdateBannerMutation();
  const [deleteBanner, {error: deleteError}] = useAdminDeleteBannerMutation();

  const [editorInstance, setEditorInstance]
    = useState<editor.IStandaloneCodeEditor | undefined>(undefined);
  const [existingBanner, setExistingBanner] = useState<Banner | undefined>(undefined);
  const [addSuccess, setAddSuccess] = useState<boolean>(false);
  const [updateSuccess, setUpdateSuccess] = useState<boolean>(false);
  const [deleteSuccess, setDeleteSuccess] = useState<boolean>(false);
  const {bannerVariables, insertVariable} = useBannerVariablesInEditor(editorInstance);
  const {handleSubmit, control, setValue} = useForm<Banner>();

  useEffect(() => {
    setExistingBanner(data);
  }, [data]);

  useEffect(() => {
    if (addSuccess) {
      toastSuccess(t('banners.createSuccess'));
      setAddSuccess(false);
    } else if (updateSuccess) {
      toastSuccess(t('banners.updateSuccess'));
      setUpdateSuccess(false);
    } else if (deleteSuccess) {
      toastSuccess(t('banners.deleteSuccess'));
      setDeleteSuccess(false);
    }
  }, [addSuccess, updateSuccess, deleteSuccess, t]);

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
      setValue('name', existingBanner.name);
      setValue('content', existingBanner.content);
    }
  }, [existingBanner, setValue]);

  const onCreate: SubmitHandler<Banner> = async newBanner => {
    await addBanner({newBanner, exerciseId: exercise.id});
    setExistingBanner(newBanner);
    setAddSuccess(true);
  };

  const onUpdate: SubmitHandler<Banner> = async updatedBanner => {
    await updateBanner({updatedBanner, exerciseId: exercise.id});
    setExistingBanner(updatedBanner);
    setUpdateSuccess(true);
  };

  const onDelete: SubmitHandler<Banner> = async () => {
    await deleteBanner({exerciseId: exercise.id});
    setValue('name', '');
    setValue('content', '');
    setExistingBanner(undefined);
    setDeleteSuccess(true);
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
            field: {onChange, value}, fieldState: {error},
          }) => (
            <FormGroup
              helperText={error?.message}
              label={
                <div className='flex justify-between items-end'>
                  <Label>
                    {t('banners.content')}
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
                  defaultLanguage='markdown'
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
        <div className='flex justify-end mt-[2rem] gap-[2rem]'>
          <Button
            large
            className='gap-[2rem]'
            type='submit'
            intent='primary'
            text={existingBanner ? t('update') : t('create')}
          />
          <Button
            large
            className='gap-[2rem]'
            type='reset'
            intent='danger'
            text={t('delete')}
          />
        </div>
      </form>
    </div>
  );
};

export default BannerView;
