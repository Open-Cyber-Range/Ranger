import React from 'react';
import {useTranslation} from 'react-i18next';

const Home = () => {
  const {t} = useTranslation();

  return (
  <div style={{padding: 20}}>
    <h2>{t('homepage')}</h2>
  </div>
);
};

export default Home;
