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
  MenuItem,
} from '@blueprintjs/core';
import {
  useAdminGetDeploymentGroupsQuery,
  useAdminGetGroupsQuery,
} from 'src/slices/apiSlice';
import {useTranslation} from 'react-i18next';
import {Controller, useForm} from 'react-hook-form';
import {Suggest2} from '@blueprintjs/select';
import {MenuItem2} from '@blueprintjs/popover2';
import {type AdGroup} from 'src/models/groups';
import DatePicker from 'react-datepicker';
import {useState} from 'react';

const AddDialog = (
  {isOpen, title, onSubmit, onCancel}:
  {
    title: string;
    isOpen: boolean;
    onSubmit: ({
      count,
      name,
      deploymentGroup,
      groupName,
      start,
      end,
    }: DeploymentForm) => void;
    onCancel: () => void;
  },
) => {
  const {t} = useTranslation();
  const {data: deployers} = useAdminGetDeploymentGroupsQuery();
  const {data: groups} = useAdminGetGroupsQuery();
  const [startDate, setStartDate] = useState<Date | undefined>(undefined);
  const [endDate, setEndDate] = useState<Date | undefined>(undefined);

  const {handleSubmit, control, register, formState: {errors}}
  = useForm<DeploymentForm>({
    defaultValues: {
      name: '',
      deploymentGroup: undefined,
      count: 1,
      start: undefined,
      end: undefined,
    },
  });

  const onHandleSubmit = (formContent: DeploymentForm) => {
    if (onSubmit) {
      onSubmit(formContent);
    }
  };

  if (isOpen !== undefined) {
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
              name='start'
              rules={{required: t('deployments.form.startDate.required') ?? ''}}
              render={({
                field: {onChange, onBlur, ref}, fieldState: {error},
              }) => {
                const intent = error ? Intent.DANGER : Intent.NONE;
                return (
                  <FormGroup
                    labelFor='start-date'
                    labelInfo='(required)'
                    helperText={error?.message}
                    intent={intent}
                    label={t('deployments.form.startDate.title')}
                  >
                    <div className='flex flex-col'>
                      <DatePicker
                        ref={ref}
                        selectsStart
                        showTimeSelect
                        customInput={<input className='bp4-input bp4-large bp4-fill'/>}
                        id='start-date'
                        selected={startDate}
                        startDate={startDate}
                        endDate={endDate}
                        timeFormat='HH:mm'
                        dateFormat='dd/MM/yyyy HH:mm'
                        onChange={date => {
                          setStartDate(date ?? undefined);
                          onChange(date?.toISOString() ?? '');
                        }}
                        onBlur={onBlur}
                      />
                    </div>
                  </FormGroup>
                );
              }}
            />
            <Controller
              control={control}
              name='end'
              rules={{
                required: t('deployments.form.endDate.required') ?? '',
                validate: {
                  endDateAfterStartDate: (value: string) =>
                    !startDate || !value || new Date(value) > new Date(startDate)
                    || (t('deployments.form.endDate.earlierThanStart') ?? ''),
                },
              }}
              render={({
                field: {onChange, onBlur, ref}, fieldState: {error},
              }) => {
                const intent = error ? Intent.DANGER : Intent.NONE;
                const filterFromStart = (time: Date) => {
                  if (startDate) {
                    return time > startDate;
                  }

                  return true;
                };

                return (
                  <FormGroup
                    labelFor='end-date'
                    labelInfo='(required)'
                    helperText={error?.message}
                    intent={intent}
                    label={t('deployments.form.endDate.title')}
                  >
                    <div className='flex flex-col'>
                      <DatePicker
                        ref={ref}
                        selectsEnd
                        showTimeSelect
                        customInput={<input className='bp4-input bp4-large bp4-fill'/>}
                        id='end-date'
                        selected={endDate}
                        startDate={startDate}
                        endDate={endDate}
                        minDate={startDate}
                        timeFormat='HH:mm'
                        dateFormat='dd/MM/yyyy HH:mm'
                        filterTime={filterFromStart}
                        onChange={date => {
                          setEndDate(date ?? undefined);
                          onChange(date?.toISOString() ?? '');
                        }}
                        onBlur={onBlur}
                      />
                    </div>
                  </FormGroup>
                );
              }}
            />
            <Controller
              control={control}
              name='groupName'
              render={({
                field: {onBlur, ref, value, onChange}, fieldState: {error},
              }) => {
                const intent = error ? Intent.DANGER : Intent.NONE;
                const activeItem = groups?.find(group => group.name === value);
                return (
                  <FormGroup
                    labelFor='deployment-group'
                    helperText={error?.message}
                    intent={intent}
                    label={t('common.adGroup')}
                  >
                    <Suggest2<AdGroup>
                      inputProps={{
                        id: 'deployment-group',
                        onBlur,
                        inputRef: ref,
                        placeholder: '',
                      }}
                      activeItem={activeItem}
                      inputValueRenderer={item => item.name}
                      itemPredicate={(query, item) =>
                        item.name.toLowerCase().includes(query.toLowerCase())}
                      itemRenderer={(item, {handleClick, handleFocus}) => (
                        <MenuItem2
                          key={item.id}
                          text={item.name}
                          onClick={handleClick}
                          onFocus={handleFocus}
                        />
                      )}
                      items={groups ?? []}
                      noResults={
                        <MenuItem
                          disabled
                          text={t('common.noResults')}
                          roleStructure='listoption'/>
                      }
                      onItemSelect={item => {
                        const event = {
                          target: {
                            value: item.name,
                          },
                        };
                        onChange(event);
                      }}
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
