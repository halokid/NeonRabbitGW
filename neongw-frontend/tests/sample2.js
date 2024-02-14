import React, { useState } from 'react';
import './Login.css'; // 引入CSS文件
import API_ENDPOINT from './config';
import { toast } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';
import { useNavigate } from 'react-router-dom';


const Login = () => {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');

  const [loggedIn, setLoggedIn] = useState(false);
  const navigate = useNavigate();


  const handleUsernameChange = (event) => {
    setUsername(event.target.value);
  };

  const handlePasswordChange = (event) => {
    setPassword(event.target.value);
  };

  const handleSubmit = async (event) => {
    event.preventDefault();

    try {
      const response = await fetch( API_ENDPOINT + '/mgt/login', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ username, password })
      });

      if (!response.ok) {
        throw new Error('Invalid credentials');
      }

      // Parse JSON response
      const rsp = await response.json();

      // Use response data
      console.log(rsp);

      if (rsp.code == 0) {
        // Show success notification
        toast.success('Login successful');
        // return <Navigate to="/service-list" />;
        setLoggedIn(true);
        console.log(loggedIn)
      } else {
        toast.success('Login fail');
      }

      // Proceed with login process
    } catch (error) {
      setError(error.message);
    }

    // return <Navigate to="/service-list" />;
    // navigate('/service-list');
  };

  // Use useEffect to perform side effects after the component has rendered
  React.useEffect(() => {
    if (loggedIn) {
      // Redirect to '/service-list' after successful login
      navigate('/service-list');
    }
  }, [loggedIn, navigate]); // Run effect when loggedIn or navigate changes

  return (
    <div className="login-container">
      <h2>Login Page</h2>
      {error && <div className="error-message">{error}</div>}
      <form className="login-form" onSubmit={handleSubmit}>
        <div>
          <label>Username:</label>
          <input
            type="text"
            value={username}
            onChange={handleUsernameChange}
            required
          />
        </div>
        <div>
          <label>Password:</label>
          <input
            type="password"
            value={password}
            onChange={handlePasswordChange}
            required
          />
        </div>
        <button type="submit">Login</button>
      </form>
    </div>
  );
};

export default Login;
