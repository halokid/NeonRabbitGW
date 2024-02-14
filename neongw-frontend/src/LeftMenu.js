
import React from 'react';
import { Link } from 'react-router-dom';

const LeftMenu = () => {
  return (
    <div className="left-menu bg-light">
      <ul className="nav flex-column">
        <li className="nav-item"><Link to="/service-list" className="nav-link">Service List</Link></li>
        {/* Add more menu items here */}
      </ul>
    </div>
  );
};

export default LeftMenu;


