import React from 'react';
import {Link} from 'react-router-dom';
import {Alignment, Navbar} from '@blueprintjs/core';

const MainNavbar = () => (
  <Navbar fixedToTop>
    <Navbar.Group align={Alignment.LEFT}>
      <Navbar.Heading>Ranger</Navbar.Heading>
      <Navbar.Divider/>
      <Link role='button' className='bp4-button bp4-minimal bp4-icon-home' to='/'>
        Home
      </Link>
      <Link role='button' className='bp4-button bp4-minimal bp4-icon-document' to='/exercises'>
        Exercises
      </Link>
    </Navbar.Group>
  </Navbar>
);

export default MainNavbar;
