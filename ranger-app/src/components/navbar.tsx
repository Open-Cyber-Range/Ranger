import React from 'react';
import { Link } from "react-router-dom";

const Navbar = () => {
    return (
        <nav style={{ margin: 10 }}>
            <Link to="/" style={{ padding: 5 }}>
                Home
            </Link>
            <Link to="/exercise" style={{ padding: 5 }}>
                Exercise
            </Link>
        </nav>
    );
}

export default Navbar;