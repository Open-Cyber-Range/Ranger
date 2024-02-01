import type React from 'react';
import {Link} from 'react-router-dom';
import {Alignment, Navbar} from '@blueprintjs/core';
import {useTranslation} from 'react-i18next';
import {ReactComponent as Logo} from 'src/assets/logos/RANGER_WHITE_64x64.svg';
import LoginInfo from './LoginInfo';
import NavbarSponsors from './SponsorIcons';

const MainNavbar: React.FC<{navbarLinks?: JSX.Element}> = ({navbarLinks}) => {
  const {t} = useTranslation();
  return (
    <Navbar fixedToTop className='h-16 flex items-center justify-between'>
      <Navbar.Group align={Alignment.LEFT}>
        <Navbar.Heading>
          <Link
            to='/'
            className='py-4 text-m font-bold uppercase tracking-wider text-gray-600'
            style={{textDecoration: 'none'}}
          >
            <div className='flex flex-col items-center mt-1'>
              <Logo className='h-10 w-10'/>
              {t('appName')}
            </div>
          </Link>
        </Navbar.Heading>
        <Navbar.Divider/>
        <NavbarSponsors/>
        <Navbar.Divider/>
        {navbarLinks}
      </Navbar.Group>
      <LoginInfo/>
    </Navbar>
  );
};

export default MainNavbar;
