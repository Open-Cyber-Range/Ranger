import {Button, Callout, H5, Icon} from '@blueprintjs/core';
import {useKeycloak} from '@react-keycloak/web';
import React, {useState} from 'react';
import {useTranslation} from 'react-i18next';
import NameDialog from 'src/components/NameDialog';
import {type Order} from 'src/models/order';
import {useClientAddOrderMutation} from 'src/slices/apiSlice';

const OrderList = () => {
  const orders: Order[] = [];
  const {t} = useTranslation();
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  const [addOrder, _newOrder] = useClientAddOrderMutation();
  const {keycloak} = useKeycloak();

  return (
    <div className='flex flex-col gap-8'>
      {orders.length === 0 && (
        <Callout icon={null} className='mt-8 flex items-center justify-between' intent='primary'>
          <div className='flex items-end'>
            <Icon icon='info-sign' className='mr-2'/>
            <H5 className='leading-[normal]'>{t('orders.noOrdersCallout')}</H5>
          </div>
          <Button
            large
            intent='success'
            onClick={() => {
              setIsDialogOpen(true);
            }}
          >{t('orders.createOrder')}
          </Button>
        </Callout>
      )}
      <NameDialog
        isOpen={isDialogOpen}
        title={t('orders.newOrder')}
        onCancel={() => {
          setIsDialogOpen(false);
        }}
        onSubmit={async name => {
          const userInfo = await keycloak.loadUserInfo() as {email?: string};
          if (userInfo.email) {
            await addOrder({
              name,
              clientId: userInfo.email,
            });
          }

          setIsDialogOpen(false);
        }}/>
      {orders.length > 1 && orders.map(order => (
        <span key={order.id}>{JSON.stringify(order)}</span>
      ))}
    </div>
  );
};

export default OrderList;
