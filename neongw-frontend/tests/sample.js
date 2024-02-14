import React from 'react';
import { TextField, Button, Container, Typography, Grid } from '@mui/material';

const Login = () => {
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
              />
            </Grid>
            <Grid item xs={12}>
              <TextField
                fullWidth
                label="Password"
                variant="outlined"
                size="small"
                type="password"
              />
            </Grid>
            <Grid item xs={12}>
              <Button
                fullWidth
                variant="contained"
                color="primary"
                size="large"
              >
                Login
              </Button>
            </Grid>
          </Grid>
        </form>
      </div>
    </Container>
  );
};

export default Login;
