import type React from 'react';
import {useState} from 'react';
import {Card, H2} from '@blueprintjs/core';
import {useNavigate} from 'react-router-dom';
import {useTranslation} from 'react-i18next';
import {type Order} from 'src/models/order';

const OrderCard = ({order}: {order: Order}) => {
  const {t} = useTranslation();
  const navigate = useNavigate();
  const [isPopoverOpen, setIsPopoverOpen] = useState(false);

  //   Const handleCardClick = () => {
  //     if (!isLoading) {
  //       navigate(exercise.id);
  //     }
  //   };

  //   useEffect(() => {
  //     if (data) {
  //       toastSuccess(t('exercises.deleteSuccess', {
  //         exerciseName: exercise.name,
  //       }));
  //     }
  //   }, [data, exercise.name, t]);

  //   useEffect(() => {
  //     if (error) {
  //       toastWarning(t('exercises.deleteFail', {
  //         exerciseName: exercise.name,
  //       }));
  //     }
  //   }, [error, exercise.name, t]);

  //   const onMouseOver = () => {
  //     if (deploymentsExist) {
  //       setIsPopoverOpen(!isPopoverOpen);
  //     }
  //   };

  return (
    <Card
      interactive
      elevation={2}
      onClick={() => {
        console.log('clicked');
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
