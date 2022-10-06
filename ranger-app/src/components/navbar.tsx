import React from 'react';
import { Link } from "react-router-dom";
import { Alignment, Navbar as Nav } from '@blueprintjs/core';

const Navbar = () => {
    return (
        <Nav fixedToTop={true}>
            <Nav.Group align={Alignment.LEFT}>
                <Nav.Heading>Ranger</Nav.Heading>
                <Nav.Divider />
                <Link role="button" className="bp4-button bp4-minimal bp4-icon-home" to="/">
                    Home
                </Link>
                <Link role="button" className="bp4-button bp4-minimal bp4-icon-document" to="/exercise">
                    Exercise
                </Link>
            </Nav.Group>
        </Nav >
    );
}

export default Navbar;