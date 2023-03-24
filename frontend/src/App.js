import logo from './logo.svg';
import './App.css';
import { useState } from 'react';
import '@mui/material/Button'
import { Container, Typography, TextField } from '@mui/material';

const App = () => {

  return (
    <Container className="App" maxWidth='xs'>
      <RouteSelection></RouteSelection>
      <CommuteTypeSelection></CommuteTypeSelection>
      {/* TODO: Car selection is only shown when car has been chosen as one of the commute choices */}
      <CarSelection></CarSelection>
      {/* Button to confirm */}
      {/* Result screen */}
      </Container>
  );
}

const RouteSelection = () => {
  return(
    <Container margin='normal'>
      <Typography align='left' variant='h6'>Your Addresses</Typography>
      <TextField margin='normal' fullWidth='90%'
        id="homeAddress"
        label="Home Address"
      />
      <br />
      <TextField margin='normal' fullWidth='90%'
      id="workAddress"
      label="Work Address"
    />
    </Container>
  );
}

const CommuteTypeSelection = () => {
  return(
    <Container>
      <Typography align='left' variant='h6'>Your Week</Typography>
    </Container>
  );
}

const CarSelection = () => {
  return(
    <Container>

    </Container>
  );
}



export default App;
