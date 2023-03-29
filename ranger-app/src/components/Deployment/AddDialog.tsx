import type React from 'react';
import type {DeploymentForm} from 'src/models/deployment';
import {
  Button,
  Dialog,
  H2,
  HTMLSelect,
  InputGroup,
  FormGroup,
  Classes,
  Label,
  Intent,
  NumericInput,
} from '@blueprintjs/core';
import {useGetDeploymentGroupsQuery} from 'src/slices/apiSlice';
import {useTranslation} from 'react-i18next';
import {Controller, useForm} from 'react-hook-form';

const AddDialog = (
  {isOpen, title, onSubmit, onCancel}:
  {
    title: string;
    isOpen?: boolean;
    onSubmit?: ({
      count,
      name,
      deploymentGroup,
    }: DeploymentForm) => void;
    onCancel?: () => void;
  },
) => {
  const {t} = useTranslation();
  const {data: deployers} = useGetDeploymentGroupsQuery();

  const {handleSubmit, control, register, formState: {errors}}
  = useForm<DeploymentForm>({
    defaultValues: {
      name: '',
      deploymentGroup: undefined,
      count: 1,
    },
  });

  const onHandleSubmit = (formContent: DeploymentForm) => {
    if (onSubmit) {
      onSubmit(formContent);
    }
  };

  if (isOpen !== undefined && onSubmit && onCancel) {
    return (
      <Dialog isOpen={isOpen}>
        <div className={Classes.DIALOG_HEADER}>
          <H2>{title}</H2>
          <Button
            small
            minimal
            icon='cross'
            onClick={() => {
              onCancel();
            }}/>
        </div>
        <form onSubmit={handleSubmit(onHandleSubmit)}>
          <div className={Classes.DIALOG_BODY}>
            <Controller
              control={control}
              name='deploymentGroup'
              render={({
                field: {onChange, onBlur, value}, fieldState: {error},
              }) => {
                const intent = error ? Intent.DANGER : Intent.NONE;
                return (
                  <FormGroup
                    labelFor='deployment-group'
                    labelInfo='(required)'
                    helperText={error?.message}
                    intent={intent}
                    label={t('deployments.form.group.title')}
                  >
                    <Label>
                      <HTMLSelect
                        {...register('deploymentGroup',
                          {required: true})}
                        autoFocus
                        large
                        fill
                        id='deployment-group'
                        value={value}
                        defaultValue=''
                        onBlur={onBlur}
                        onChange={onChange}
                      >
                        <option disabled hidden value=''>
                          {t('deployments.form.group.placeholder')}
                        </option>
                        {Object.keys((deployers ?? {})).map(groupName =>
                          <option key={groupName}>{groupName}</option>)}
                      </HTMLSelect>
                      {errors.deploymentGroup && (
                        <span className='text-xs text-red-800'>
                          {t('deployments.form.group.required')}
                        </span>
                      ) }
                    </Label>
                  </FormGroup>
                );
              }}
            />
            <Controller
              control={control}
              name='name'
              rules={{required: t('deployments.form.name.required') ?? ''}}
              render={({
                field: {onChange, onBlur, ref, value}, fieldState: {error},
              }) => {
                const intent = error ? Intent.DANGER : Intent.NONE;
                return (
                  <FormGroup
                    labelFor='deployment-name'
                    labelInfo='(required)'
                    helperText={error?.message}
                    intent={intent}
                    label={t('deployments.form.name.title')}
                  >
                    <InputGroup
                      large
                      intent={intent}
                      value={value}
                      inputRef={ref}
                      id='deployment-name'
                      onChange={onChange}
                      onBlur={onBlur}
                    />
                  </FormGroup>
                );
              }}
            />
            <Controller
              control={control}
              name='count'
              rules={{required: t('deployments.form.count.required') ?? ''}}
              render={({
                field: {onChange, onBlur, ref, value}, fieldState: {error},
              }) => {
                const intent = error ? Intent.DANGER : Intent.NONE;
                return (
                  <FormGroup
                    labelFor='deployment-count'
                    labelInfo='(required)'
                    helperText={error?.message}
                    intent={intent}
                    label={t('deployments.form.count.title')}
                  >
                    <NumericInput
                      fill
                      large
                      buttonPosition='none'
                      max={200}
                      min={1}
                      intent={intent}
                      value={value}
                      inputRef={ref}
                      id='deployment-count'
                      onValueChange={onChange}
                      onBlur={onBlur}
                    />
                  </FormGroup>
                );
              }}
            />
          </div>
          <div className={Classes.DIALOG_FOOTER}>
            <div className={Classes.DIALOG_FOOTER_ACTIONS}>
              <Button
                large
                type='submit'
                intent='primary'
                text={t('common.add')}
              />
            </div>
          </div>
        </form>
      </Dialog>
    );
  }

  return null;
};

export default AddDialog;
