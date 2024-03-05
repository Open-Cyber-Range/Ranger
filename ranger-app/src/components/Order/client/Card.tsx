import type React from 'react';
import {Card, H2, Tag} from '@blueprintjs/core';
import {useNavigate} from 'react-router-dom';
import {type Order} from 'src/models/order';
import {useTranslation} from 'react-i18next';
import {type TFunction} from 'i18next';

const orderToTag = (order: Order, t: TFunction) => {
  switch (order.status) {
    case 'draft': {
      return <Tag minimal round>{t('orders.statuses.draft')}</Tag>;
    }

    case 'review': {
      return <Tag minimal round intent='primary'>{t('orders.statuses.review')}</Tag>;
    }

    case 'inprogress': {
      return <Tag minimal round intent='warning'>{t('orders.statuses.inprogress')}</Tag>;
    }

    case 'ready': {
      return <Tag minimal round intent='success'>{t('orders.statuses.ready')}</Tag>;
    }

    case 'finished': {
      return <Tag minimal round>{t('orders.statuses.finished')}</Tag>;
    }

    default: {
      return null;
    }
  }
};

const OrderCard = ({order}: {order: Order}) => {
  const navigate = useNavigate();
  const {t} = useTranslation();

  return (
    <Card
      interactive
      elevation={2}
      onClick={() => {
        navigate(`/orders/${order.id}`);
      }}
    >
      <div className='flex flex-row justify-between'>
        <H2>{order.name}</H2>
        {orderToTag(order, t)}
      </div>
    </Card>
  );
};

export default OrderCard;
