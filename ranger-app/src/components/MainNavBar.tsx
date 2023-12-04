import React from 'react';
import {Link} from 'react-router-dom';
import {Alignment, Navbar} from '@blueprintjs/core';
import {useTranslation} from 'react-i18next';
import LoginInfo from './LoginInfo';
import NavbarSponsors from './NavbarSponsors';

const MainNavbar = () => {
  const {t} = useTranslation();
  return (
    <Navbar fixedToTop className='h-16 flex items-center'>
      <Navbar.Group align={Alignment.LEFT}>
        <Navbar.Heading>
          <Link
            to='/'
            className='py-4 text-m font-bold uppercase tracking-wider text-gray-600'
            style={{textDecoration: 'none'}}
          >
            {t('appName')}
          </Link>
        </Navbar.Heading>
        <Navbar.Divider/>
        <NavbarSponsors/>
        <Navbar.Divider/>
        <Link
          role='button'
          className='bp4-button bp4-minimal bp4-icon-document'
          to='/exercises'
        >
          {t('menu.exercises')}
        </Link>
        <Link
          role='button'
          className='bp4-button bp4-minimal bp4-icon-label'
          to='/logs'
        >
          {t('menu.logs')}
        </Link>
      </Navbar.Group>
      <a className='flex-grow'>
        <LoginInfo/>
      </a>
    </Navbar>
  );
};

export default MainNavbar;
