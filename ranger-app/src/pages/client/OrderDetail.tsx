import {Breadcrumbs, H2, Tag} from '@blueprintjs/core';
import React from 'react';
import {useTranslation} from 'react-i18next';
import {useParams} from 'react-router-dom';
import PageHolder from 'src/components/PageHolder';
import {type OrderDetailRouteParamaters} from 'src/models/routes';
import {useClientGetOrderQuery} from 'src/slices/apiSlice';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import PageLoader from 'src/components/PageLoader';
import {getBreadcrumIntent} from 'src/utils';
import StepFooter from 'src/components/Order/client/StepFooter';
import TrainingObjectives from 'src/components/Order/client/TrainingObjectives';
import Structure from 'src/components/Order/client/Structure';
import Environment from 'src/components/Order/client/Environment';
import {type Order} from 'src/models/order';
import CustomElements from 'src/components/Order/client/CustomElements';
import Plot from 'src/components/Order/client/Plot';

function readyForNext(formType: string, order: Order | undefined): boolean {
  return (formType === 'training-objectives' && (order?.trainingObjectives?.length ?? 0) > 0)
  || (formType === 'structure' && (order?.structures?.length ?? 0) > 0)
  || (formType === 'environment' && (order?.environments?.length ?? 0) > 0)
  || formType === 'custom-elements'
  || formType === 'plot';
}

const OrderDetail = () => {
  const {t} = useTranslation();
  const {orderId, stage} = useParams<OrderDetailRouteParamaters>();
  const formType = stage ?? 'training-objectives';
  const {data: order, isLoading} = useClientGetOrderQuery(orderId ?? skipToken);

  if (isLoading) {
    return (
      <PageLoader title={t('orders.loadingOrder')}/>
    );
  }

  return (
    <PageHolder>
      <H2>{t('orders.order')}: {order?.name}</H2>
      <div className='my-4'>
        {orderId && (
          <StepFooter
            readyForNext={readyForNext(formType, order)}
            orderId={orderId}
            stage={formType}
            onSubmit={() => {
              // Console.log('sumbitted');
            }}/>
        )}
      </div>
      <Breadcrumbs
        className='mt-4'
        breadcrumbRenderer={({icon, intent, text}) => (
          <Tag
            large
            round
            minimal
            icon={icon}
            intent={intent}

          >{text}
          </Tag>
        )}
        items={[
          {
            href: 'training-objectives',
            icon: 'new-object',
            text: t('orders.trainingObjectives'),
            intent: getBreadcrumIntent('training-objectives', formType),
          },
          {
            href: 'structure',
            icon: 'many-to-many',
            text: t('orders.structure'),
            intent: getBreadcrumIntent('structure', formType),
          },
          {
            href: 'environment',
            icon: 'globe-network',
            text: t('orders.environment'),
            intent: getBreadcrumIntent('environment', formType),
          },
          {
            href: 'custom-elements',
            icon: 'detection',
            text: t('orders.customElements'),
            intent: getBreadcrumIntent('custom-elements', formType),
          },
          {
            href: 'plot',
            icon: 'manual',
            text: t('orders.plot'),
            intent: getBreadcrumIntent('plot', formType),
          },
        ]}/>
      <div className='mt-4 min-h-full'>
        {order && formType === 'training-objectives' && (<TrainingObjectives order={order}/>)}
        {order && formType === 'structure' && (<Structure order={order}/>)}
        {order && formType === 'environment' && (<Environment order={order}/>)}
        {order && formType === 'custom-elements' && (<CustomElements order={order}/>)}
        {order && formType === 'plot' && (<Plot order={order}/>)}
      </div>
    </PageHolder>
  );
};

export default OrderDetail;
