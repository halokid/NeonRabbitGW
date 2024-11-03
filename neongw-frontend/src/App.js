import {BrowserRouter as Router, Navigate, Route, Routes} from 'react-router-dom';
import React from 'react';
import Login from './Login';
import ServiceList from './ServiceList';
// import LeftMenu from "./LeftMenu";
import PrivateRoute from "./PrivateRoute";
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
        {/* 默认重定向到 ServiceList（受保护）页面 */}
        <Route path="/" element={<Navigate to="/service-list" />} />

        {/* 受保护的路由 */}
        <Route element={<PrivateRoute />}>
          <Route element={<Layout />}>
            <Route path="/service-list" element={<ServiceList />} />
          </Route>
        </Route>

        {/* 公共路由 */}
        <Route path="/login" element={<Login />} />
      </Routes>

    </Router>
  );
}

export default App;

