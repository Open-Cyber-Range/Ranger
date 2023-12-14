import React from 'react';
import {Link} from 'react-router-dom';
import {useTranslation} from 'react-i18next';

const ParticipantNavbarLinks = () => {
  const {t} = useTranslation();
  return (
    <Link
      role='button'
      className='bp4-button bp4-minimal bp4-icon-document'
      to='/exercises'
    >
      {t('menu.exercises')}
    </Link>
  );
};

export default ParticipantNavbarLinks;
