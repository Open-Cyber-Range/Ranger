import type React from 'react';
import {Card, H2} from '@blueprintjs/core';
import {useNavigate} from 'react-router-dom';
import {type Order} from 'src/models/order';

const OrderCard = ({order}: {order: Order}) => {
  const navigate = useNavigate();

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
      </div>
    </Card>
  );
};

export default OrderCard;
