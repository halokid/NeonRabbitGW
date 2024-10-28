import {BrowserRouter as Router, Navigate, Route, Routes} from 'react-router-dom';
import React from 'react';
import Login from './Login';
import ServiceList from './ServiceList';
// import LeftMenu from "./LeftMenu";
import Layout from "./Layout";
import {ToastContainer} from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';
import 'bootstrap/dist/css/bootstrap.min.css';
import './LeftMenu.css';


function App() {
  /*
  return (
    <div>
      <ToastContainer />
      <Login />
    </div>
  );
   */

  return (
    <Router>
      <ToastContainer/>
      {/*<Layout/>*/}

      <Routes>
        <Route path="/" element={<Navigate to="/login"/>}/>
        <Route path="/login" element={<Login/>}/>
        {/*<Route path="/service-list" element={<ServiceList/>}/>*/}

        <Route path="/service-list" element={<Layout/>}>
        <Route index element={<ServiceList/>}/>
        {/* Add more routes for other pages */}
        </Route>

        {/* Other routes */}
      </Routes>

    </Router>


  );
}

export default App;

