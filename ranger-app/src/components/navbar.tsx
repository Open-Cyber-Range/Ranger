import {Link} from 'react-router-dom';
import {Alignment, Navbar} from '@blueprintjs/core';

const HomePageNavbar = () => (
  <Navbar fixedToTop={true}>
    <Navbar.Group align={Alignment.LEFT}>
      <Navbar.Heading>Ranger</Navbar.Heading>
      <Navbar.Divider />
      <Link role='button' className='bp4-button bp4-minimal bp4-icon-home' to='/'>
        Home
      </Link>
      <Link role='button' className='bp4-button bp4-minimal bp4-icon-document' to='/exercise'>
        Exercise
      </Link>
    </Navbar.Group>
  </Navbar >
);

export default HomePageNavbar;
