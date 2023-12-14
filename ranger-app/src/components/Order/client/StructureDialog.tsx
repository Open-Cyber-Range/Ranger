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
import {type Order, type NewStructure, type Structure} from 'src/models/order';

const StructureDialog = (
  {
    isOpen,
    crossClicked,
    onSubmit,
    editableStructure,
    order,
  }: {
    isOpen: boolean;
    crossClicked: () => void;
    onSubmit: (formContent: NewStructure) => void;
    editableStructure?: Structure;
    order: Order;
  },
) => {
  const {t} = useTranslation();

  const {handleSubmit, control, reset} = useForm<NewStructure>({
    defaultValues: {
      name: '',
      description: '',
      parentId: undefined,
    },
  });

  useEffect(() => {
    reset({
      name: editableStructure?.name ?? '',
      description: editableStructure?.description ?? '',
      parentId: editableStructure?.parentId ?? '',
    });
  }, [editableStructure, reset]);

  const onHandleSubmit = async (formContent: NewStructure) => {
    const newStructure = {
      ...formContent,
      parentId: formContent.parentId === '' ? undefined : formContent.parentId,
    };
    onSubmit(newStructure);
    reset();
  };

  const structuresExist = order.structures && order.structures.length > 0;

  return (
    <Dialog
      isOpen={isOpen}
    >
      <div className={Classes.DIALOG_HEADER}>
        <H2>{t('orders.structureElements.add')}</H2>
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
          <DialogTextInput<NewStructure>
            controllerProps={{
              control,
              name: 'name',
              rules: {
                required: t('orders.structureElements.nameRequired') ?? '',
                maxLength: {
                  value: 255,
                  message: t('orders.structureElements.nameMaxLength'),
                },
              },
            }}
            id='name'
            label={t('orders.structureElements.name')}
          />
          <DialogTextArea<NewStructure>
            textAreaProps={{
              fill: true,
              autoResize: true,
            }}
            controllerProps={{
              control,
              name: 'description',
              rules: {
                required: t('orders.structureElements.descriptionRequired') ?? '',
                maxLength: {
                  value: 3000,
                  message: t('orders.structureElements.descriptionMaxLength'),
                },
              },
            }}
            id='description'
            label={t('orders.structureElements.description')}
          />
          <DialogSelect<NewStructure>
            selectProps={{
              disabled: !structuresExist,
              fill: true,
              options: structuresExist ? [
                {
                  label: t('orders.structureElements.noParent') ?? '',
                  value: '',
                },
                ...(order.structures?.map(structure => ({
                  label: structure.name,
                  value: structure.id,
                })) ?? []),
              ] : [{
                label: t('orders.structureElements.noPossibleParents') ?? '',
                value: '',
              }],
            }}
            controllerProps={{
              control,
              name: 'parentId',
            }}
            id='parentId'
            label={t('orders.structureElements.parent')}
          />
          <div className='flex justify-end'>
            {/* <div className='flex gap-2'>
              <Button
                minimal
                intent='primary'
                icon='plus'
                onClick={() => {
                  append({threat: ''});
                }}
              >
                {t('orders.trainingObjective.addNewThreat')}
              </Button>
            </div> */}
          </div>
          {/* {fields.map((field, index) => (
            <div key={field.id} className='flex gap-6 items-end'>
              <div className='grow'>
                <DialogTextInput<NewTrainingObjective>
                  controllerProps={{
                    control,
                    name: `threats.${index}.threat`,
                    rules: {
                      required: t('orders.trainingObjective.threatRequired') ?? '',
                      maxLength: {
                        value: 255,
                        message: t('orders.threatMaxLength.maxLength'),
                      },
                    },
                    defaultValue: '',
                  }}
                  id={`threats.${index}`}
                  label={t('orders.trainingObjective.threat')}
                />
              </div>
              <Button
                minimal
                intent='danger'
                className='shrink-0 my-6'
                icon='remove'
                onClick={() => {
                  remove(index);
                }}/>
            </div>
          ))} */}
        </DialogBody>
        <DialogFooter
          actions={<Button intent='primary' type='submit' text={t('orders.submit')}/>}
        />
      </form>
    </Dialog>
  );
};

export default StructureDialog;

