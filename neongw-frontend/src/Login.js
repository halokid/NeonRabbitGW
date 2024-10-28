import { TextField, Button, Container, Typography, Grid, Snackbar } from '@mui/material';
import React, { useState } from 'react';
// import './Login.css'; // 引入CSS文件
import API_ENDPOINT from './config';
import { toast } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';
import { useNavigate } from 'react-router-dom';


const Login = () => {

  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const [openSnackbar, setOpenSnackbar] = useState(false);

  const [loggedIn, setLoggedIn] = useState(false);
  const navigate = useNavigate();

  const handleSnackbarClose = () => {
    setOpenSnackbar(false);
  };

  const handleUsernameChange = (event) => {
    setUsername(event.target.value);
  };

  const handlePasswordChange = (event) => {
    setPassword(event.target.value);
  };

  const handleSubmit = async (event) => {
    event.preventDefault();

    if (!username.trim() || !password.trim()) {
      setError('Username or password cannot be empty.');
      setOpenSnackbar(true);
      return;
    }

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
        toast.error('Login fail');
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
    <Container maxWidth="xs">
      <div style={{ marginTop: '100px' }}>
        <Typography variant="h4" align="center" gutterBottom>
          Login
        </Typography>
        <form>
          <Grid container spacing={2}>
            <Grid item xs={12}>
              <TextField
                fullWidth
                label="Username"
                variant="outlined"
                size="small"
                onChange={handleUsernameChange}
                required
              />
            </Grid>
            <Grid item xs={12}>
              <TextField
                fullWidth
                label="Password"
                variant="outlined"
                size="small"
                type="password"
                onChange={handlePasswordChange}
                required
              />
            </Grid>

            {error && (
              <Grid item xs={12}>
                <Typography variant="body2" color="error">
                  {error}
                </Typography>
              </Grid>
            )}

            <Grid item xs={12}>
              <Button
                fullWidth
                variant="contained"
                color="primary"
                size="large"
                onClick={handleSubmit}
              >
                Login
              </Button>
            </Grid>
          </Grid>
        </form>
        <Snackbar
          open={openSnackbar}
          autoHideDuration={3000}
          onClose={handleSnackbarClose}
          message={error}
        />
      </div>
    </Container>
  );
};

export default Login;

