import React from 'react';
import {Link} from 'react-router-dom';
import {Alignment, Navbar} from '@blueprintjs/core';
import {useTranslation} from 'react-i18next';

const MainNavbar = () => {
  const {t} = useTranslation();
  return (
    <Navbar fixedToTop>
      <Navbar.Group align={Alignment.LEFT}>
        <Navbar.Heading>{t('appName')}</Navbar.Heading>
        <Navbar.Divider/>
        <Link
          role='button'
          className='bp4-button bp4-minimal bp4-icon-home'
          to='/'
        >
          {t('menu.home')}
        </Link>
        <Link
          role='button'
          className='bp4-button bp4-minimal bp4-icon-document'
          to='/exercises'
        >
          {t('menu.exercises')}
        </Link>
      </Navbar.Group>
    </Navbar>
  );
};

export default MainNavbar;
