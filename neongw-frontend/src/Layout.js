import React from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { Drawer, List, ListItem, ListItemText, Container } from '@mui/material';
import ServiceList from "./ServiceList";
import {Route} from "react-router-dom";
import { Outlet } from 'react-router-dom';

const Layout = () => {
  // log out
  const navigate = useNavigate();
  const handleLogout = () => {
    localStorage.removeItem('isLoggedIn');
    navigate('/login');
  };

  return (
    <div style={{ display: 'flex' }}>
      <Drawer
        sx={{
          width: 240,
          flexShrink: 0,
          '& .MuiDrawer-paper': {
            width: 240,
            boxSizing: 'border-box',
          },
        }}
        variant="permanent"
        anchor="left"
      >
        <List>
          <ListItem button component={Link} to="/service-list">
            <ListItemText primary="Services" />
          </ListItem>

          <ListItem button onClick={handleLogout}>
            <ListItemText primary="Logout" />
          </ListItem>
        </List>
      </Drawer>
      <Container sx={{ flexGrow: 1, mt: 8, ml: 2 }}>
        {/*<h1>Main Content</h1>*/}
        {/*<p>This is the main content area.</p>*/}
        <Outlet />
      </Container>
    </div>
  );
};

export default Layout;


