import {
  Button,
  Classes,
  Dialog,
  DialogBody,
  DialogFooter,
  H2,
} from '@blueprintjs/core';
import React, {useEffect} from 'react';
import {useForm} from 'react-hook-form';
import {useTranslation} from 'react-i18next';
import DialogSelect from 'src/components/Form/DialogSelect';
import DialogTextArea from 'src/components/Form/DialogTextArea';
import DialogTextInput from 'src/components/Form/DialogTextInput';
import {
  type Order,
  type CustomElement,
  type NewCustomElement,
} from 'src/models/order';

const CustomElementDialog = (
  {
    isOpen,
    crossClicked,
    onSubmit,
    editableCustomElement,
    order,
  }: {
    isOpen: boolean;
    crossClicked: () => void;
    onSubmit: (formContent: NewCustomElement) => void;
    editableCustomElement?: CustomElement;
    order: Order;
  },
) => {
  const {t} = useTranslation();

  const {handleSubmit, control, reset} = useForm<NewCustomElement>({
    defaultValues: {
      name: '',
      description: '',
      environmentId: '',
    },
  });

  useEffect(() => {
    reset({
      name: editableCustomElement?.name ?? '',
      description: editableCustomElement?.description ?? '',
      environmentId: editableCustomElement?.environmentId ?? '',
    });
  }, [editableCustomElement, reset]);

  const onHandleSubmit = async (formContent: NewCustomElement) => {
    onSubmit(formContent);
    reset();
  };

  const environmentExists = order.environments && order.environments?.length > 0;
  return (
    <Dialog
      isOpen={isOpen}
    >
      <div className={Classes.DIALOG_HEADER}>
        <H2>{t('orders.environmentElements.add')}</H2>
        <Button
          small
          minimal
          icon='cross'
          onClick={() => {
            crossClicked();
          }}/>
      </div>
      <form onSubmit={handleSubmit(onHandleSubmit)}>
        <DialogBody>
          <DialogTextInput<NewCustomElement>
            controllerProps={{
              control,
              name: 'name',
              rules: {
                required: t('orders.customElement.nameRequired') ?? '',
                maxLength: {
                  value: 255,
                  message: t('orders.customElement.nameMaxLength'),
                },
              },
            }}
            id='name'
            label={t('orders.customElement.name')}
          />
          <DialogTextArea<NewCustomElement>
            textAreaProps={{
              fill: true,
              autoResize: true,
            }}
            controllerProps={{
              control,
              name: 'description',
              rules: {
                maxLength: {
                  value: 3000,
                  message: t('orders.customElement.descriptionMaxLength'),
                },
              },
            }}
            id='description'
            label={t('orders.customElement.description')}
          />
          <DialogSelect<NewCustomElement>
            selectProps={{
              disabled: !environmentExists,
              fill: true,
              options: environmentExists ? [
                {
                  label: t('orders.customElement.noEnvironment') ?? '',
                  value: '',
                },
                ...(order.environments?.map(environment => ({
                  label: environment.name,
                  value: environment.id,
                })) ?? []),
              ] : [{
                label: t('orders.customElement.noPossibleEnvironments') ?? '',
                value: '',
              }],
            }}
            controllerProps={{
              control,
              name: 'environmentId',
              rules: {
                required: t('orders.customElement.environmentRequired') ?? '',
              },
            }}
            id='environmentId'
            label={t('orders.customElement.environment')}
          />
        </DialogBody>
        <DialogFooter
          actions={<Button intent='primary' type='submit' text={t('orders.submit')}/>}
        />
      </form>
    </Dialog>
  );
};

export default CustomElementDialog;

