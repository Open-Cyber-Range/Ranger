import {
  Button,
  Callout,
  Classes,
  Dialog,
  DialogBody,
  DialogFooter,
  H2,
} from '@blueprintjs/core';
import React, {useEffect, useState} from 'react';
import {useForm} from 'react-hook-form';
import {useTranslation} from 'react-i18next';
import DialogTextInput from 'src/components/Form/DialogTextInput';
import {toastWarning} from 'src/components/Toaster';
import {type Order, type TrainingObjectiveForm} from 'src/models/order';
import {useClientAddTrainingObjectiveMutation} from 'src/slices/apiSlice';

const TrainingObjectives = ({order}: {order: Order}) => {
  const {t} = useTranslation();
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  const [threatNumber, setThreatNumber] = useState<number>(1);
  const threatArray = Array.from(Array.from({length: threatNumber}).keys());
  const [addTrainingObjective, {error}] = useClientAddTrainingObjectiveMutation();

  useEffect(() => {
    if (error) {
      toastWarning(t(
        'order.trainingObjective.failedtoAdd',
      ));
    }
  }, [error, t]);

  const {handleSubmit, control} = useForm<TrainingObjectiveForm>({
    defaultValues: {
      objective: '',
      threats: [],
    },
  });

  const onHandleSubmit = async (formContent: TrainingObjectiveForm) => {
    await addTrainingObjective({newTrainingObjective: formContent, orderId: order.id});
    setIsDialogOpen(false);
  };

  return (
    <>
      <Dialog
        isOpen={isDialogOpen}
      >
        <div className={Classes.DIALOG_HEADER}>
          <H2>{t('orders.trainingObjective.add')}</H2>
          <Button
            small
            minimal
            icon='cross'
            onClick={() => {
              setIsDialogOpen(false);
            }}/>
        </div>
        <form onSubmit={handleSubmit(onHandleSubmit)}>
          <DialogBody>
            <DialogTextInput<TrainingObjectiveForm>
              controllerProps={{
                control,
                name: 'objective',
                rules: {
                  required: t('orders.trainingObjective.objectiveRequired') ?? '',
                  maxLength: {
                    value: 56,
                    message: t('orders.trainingObjective.objectiveMaxLength'),
                  },
                },
              }}
              id='objective'
              label={t('orders.trainingObjective.objective')}
            />
            <div className='flex justify-end'>
              <div className='flex gap-2'>
                {threatNumber > 1 && (
                  <Button
                    minimal
                    intent='danger'
                    icon='minus'
                    onClick={() => {
                      setThreatNumber(threatNumber - 1);
                    }}
                  >
                    {t('orders.trainingObjective.removeLastThreat')}
                  </Button>
                )}
                <Button
                  minimal
                  intent='primary'
                  icon='plus'
                  onClick={() => {
                    setThreatNumber(threatNumber + 1);
                  }}
                >
                  {t('orders.trainingObjective.addNewThreat')}
                </Button>
              </div>
            </div>
            {threatArray.map((threatNumber, index) => (
              <DialogTextInput<TrainingObjectiveForm>
                key={threatNumber}
                controllerProps={{
                  control,
                  name: `threats.${index}`,
                  rules: {
                    required: t('orders.trainingObjective.threatRequired') ?? '',
                    maxLength: {
                      value: 56,
                      message: t('orders.threatMaxLength.maxLength'),
                    },
                  },
                  defaultValue: '',
                }}
                id={`threats.${index}`}
                label={t('orders.trainingObjective.threat')}
              />
            ))}
          </DialogBody>
          <DialogFooter
            actions={<Button intent='primary' type='submit' text={t('orders.submit')}/>}
          />
        </form>
      </Dialog>
      <Callout intent='primary' icon='info-sign'>
        {t('orders.trainingObjective.explenation')}
      </Callout>
      <div className='mt-4 flex gap-2 justify-between'>
        <div/>
        <Button
          large
          intent='primary'
          onClick={() => {
            setIsDialogOpen(true);
          }}
        >
          {t('orders.trainingObjective.add')}
        </Button>
      </div>
    </>
  );
};

export default TrainingObjectives;

