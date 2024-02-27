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
import {type Order, type Plot, type NewPlot} from 'src/models/order';
import {
  useClientAddPlotMutation,
  useClientDeletePlotMutation,
  useClientUpdatePlotMutation,
} from 'src/slices/apiSlice';
import {sortByProperty} from 'sort-by-property';
import PlotDialog from './PlotDialog';

const PlotElement = ({order}: {order: Order}) => {
  const {t} = useTranslation();
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  const [addPlot, {error}] = useClientAddPlotMutation();
  const [deletePlot, {error: deleteError}] = useClientDeletePlotMutation();
  const [updatePlot, {error: updateError}]
          = useClientUpdatePlotMutation();
  const [editedPlot, setEditedPlot]
          = useState<Plot | undefined>();
  const {plots: potentialPlots} = order;
  const sortedPlots = [...(potentialPlots ?? [])]
    .sort(sortByProperty('name', 'desc'));

  useEffect(() => {
    if (error) {
      toastWarning(t(
        'orders.plotElement.failedToAdd',
      ));
    }
  }, [error, t]);

  useEffect(() => {
    if (deleteError) {
      toastWarning(t(
        'orders.plotElement.failedToDelete',
      ));
    }
  }, [deleteError, t]);

  useEffect(() => {
    if (updateError) {
      toastWarning(t(
        'orders.plotElement.failedToUpdate',
      ));
    }
  }, [updateError, t]);

  const onHandleSubmit = async (formContent: NewPlot) => {
    setIsDialogOpen(false);
    if (editedPlot) {
      await updatePlot({
        newPlot: {
          ...editedPlot,
          ...formContent,
        },
        orderId: order.id,
        plotId: editedPlot.id,
      });
    } else {
      await addPlot({
        newPlot: formContent,
        orderId: order.id,
      });
    }

    setEditedPlot(undefined);
  };

  return (
    <>
      <PlotDialog
        crossClicked={() => {
          setIsDialogOpen(false);
        }}
        isOpen={isDialogOpen}
        editablePlot={editedPlot}
        order={order}
        onSubmit={onHandleSubmit}
      />
      <Callout intent='primary' icon='info-sign'>
        {t('orders.plotElement.explenation')}
      </Callout>
      <Callout className='mt-2' intent='primary' icon='info-sign'>
        {t('orders.plotElement.plotPointExplenation')}
      </Callout>
      <div className='mt-4 flex gap-4 justify-between items-start'>
        <div className='flex flex-col gap-4 grow'>
          {sortedPlots.map(plot => (
            <Card key={plot.id} className='min-w-0' elevation={Elevation.TWO}>
              <div className='flex gap-2'>
                <H3
                  className='truncate max-w-xl m-0'
                >
                  {plot.name}
                </H3>
                <Tag
                  minimal
                  round
                  icon='time'
                >
                  {plot.startTime}-{plot.endTime}
                </Tag>
              </div>
              {plot.description && (
                <div className='flex flex-wrap gap-4 mt-2'>
                  <p>{plot.description}</p>
                </div>)}
              <div className='flex mt-4 gap-2 justify-end'>
                <Button
                  intent='danger'
                  onClick={async () => {
                    await deletePlot({
                      orderId: order.id,
                      plotId: plot.id,
                    });
                  }}
                >
                  {t('common.delete')}
                </Button>
                <Button
                  intent='warning'
                  onClick={() => {
                    setEditedPlot(plot);
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
            setEditedPlot(undefined);
            setIsDialogOpen(true);
          }}
        >
          {t('orders.plotElement.add')}
        </Button>
      </div>
    </>
  );
};

export default PlotElement;
