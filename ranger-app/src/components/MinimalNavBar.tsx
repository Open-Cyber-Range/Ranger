import React from 'react';
import {Alignment, Navbar} from '@blueprintjs/core';
import {useTranslation} from 'react-i18next';
import LoginInfo from 'src/components/LoginInfo';

const MinimalNavBar = () => {
  const {t} = useTranslation();
  return (
    <Navbar fixedToTop>
      <Navbar.Group align={Alignment.LEFT}>
        <Navbar.Heading>{t('appName')}</Navbar.Heading>
      </Navbar.Group>
      <LoginInfo/>
    </Navbar>
  );
};

export default MinimalNavBar;
