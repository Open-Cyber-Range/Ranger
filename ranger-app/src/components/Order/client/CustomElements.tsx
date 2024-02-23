import {
  Button,
  Callout,
  Card,
  Elevation,
  H3,
  Tag,
} from '@blueprintjs/core';
import React, {useEffect, useState} from 'react';
import {useTranslation} from 'react-i18next';
import {toastWarning} from 'src/components/Toaster';
import {
  type Order,
  type CustomElement,
  type NewCustomElement,
} from 'src/models/order';
import {
  useClientAddCustomElementMutation,
  useClientDeleteCustomElementMutation,
  useClientUpdateCustomElementMutation,
} from 'src/slices/apiSlice';
import {sortByProperty} from 'sort-by-property';
import CustomElementDialog from './CustomElementDialog';

const CustomElements = ({order}: {order: Order}) => {
  const {t} = useTranslation();
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  const [addCustomElement, {error}] = useClientAddCustomElementMutation();
  const [deleteCustomElement, {error: deleteError}] = useClientDeleteCustomElementMutation();
  const [updateCustomElement, {error: updateError}]
        = useClientUpdateCustomElementMutation();
  const [editedCustomElement, setEditedCustomElement]
        = useState<CustomElement | undefined>();
  const {customElements: potentialCustomElements} = order;
  const sortedCustomElements = [...(potentialCustomElements ?? [])]
    .sort(sortByProperty('name', 'desc'));

  useEffect(() => {
    if (error) {
      toastWarning(t(
        'order.customElement.failedtoAdd',
      ));
    }
  }, [error, t]);

  useEffect(() => {
    if (deleteError) {
      toastWarning(t(
        'order.customElement.failedToDelete',
      ));
    }
  }, [deleteError, t]);

  useEffect(() => {
    if (updateError) {
      toastWarning(t(
        'order.customElement.failedToUpdate',
      ));
    }
  }, [updateError, t]);

  const onHandleSubmit = async (formContent: NewCustomElement) => {
    setIsDialogOpen(false);
    if (editedCustomElement) {
      await updateCustomElement({
        customElement: {
          ...editedCustomElement,
          ...formContent,
        },
        orderId: order.id,
        customElementId: editedCustomElement.id,
      });
    } else {
      await addCustomElement({
        newCustomElement: formContent,
        orderId: order.id,
      });
    }

    setEditedCustomElement(undefined);
  };

  const enviroments = order.environments ?? [];

  return (
    <>
      <CustomElementDialog
        crossClicked={() => {
          setIsDialogOpen(false);
        }}
        isOpen={isDialogOpen}
        editableCustomElement={editedCustomElement}
        order={order}
        onSubmit={onHandleSubmit}
      />
      <Callout intent='primary' icon='info-sign'>
        {t('orders.customElement.explenation')}
      </Callout>
      <div className='mt-4 flex gap-4 justify-between items-start'>
        <div className='flex flex-col gap-4 grow'>
          {sortedCustomElements.map(customElement => (
            <Card key={customElement.id} className='min-w-0' elevation={Elevation.TWO}>
              <div className='flex gap-2'>
                <H3
                  className='truncate max-w-xl m-0'
                >
                  {customElement.name}
                </H3>
                {enviroments
                  .find(environment => environment.id === customElement.environmentId)?.name ? (
                    <Tag
                      minimal
                      round
                      intent='success'
                      icon='inheritance'
                    >
                      {enviroments
                        .find(environment => environment.id === customElement.environmentId)?.name}
                    </Tag>
                  ) : null}
              </div>
              <div className='flex flex-wrap gap-4 mt-2'>
                <p>{customElement.description}</p>
              </div>
              <div className='flex mt-4 gap-2 justify-end'>
                <Button
                  intent='danger'
                  onClick={async () => {
                    await deleteCustomElement({
                      orderId: order.id,
                      customElementId: customElement.id,
                    });
                  }}
                >
                  {t('common.delete')}
                </Button>
                <Button
                  intent='warning'
                  onClick={() => {
                    setEditedCustomElement(customElement);
                    setIsDialogOpen(true);
                  }}
                >
                  {t('common.edit')}
                </Button>
              </div>
            </Card>
          ))}
        </div>
        <Button
          large
          className='shrink-0'
          intent='primary'
          onClick={() => {
            setEditedCustomElement(undefined);
            setIsDialogOpen(true);
          }}
        >
          {t('orders.customElement.add')}
        </Button>
      </div>
    </>
  );
};

export default CustomElements;
