import logo from './logo.svg';
import './App.css';
import { useState } from 'react';
import '@mui/material/Button'
import  { Container, Button, Typography, TextField } from '@mui/material';
import { border } from '@mui/system';

const App = () => {
  const [result, setResult] = useState('Shitty');
  return (
    <Container className="App" maxWidth='xs' style={{ boxShadow: '10px 10px 52px 0px rgba(0,0,0,0.39)', borderRadius: '11px', marginTop: '30px' }}>
      <RouteSelection></RouteSelection>
      <CommuteTypeSelection></CommuteTypeSelection>
      {/* TODO: Car selection is only shown when car has been chosen as one of the commute choices */}
      <CarSelection></CarSelection>
      {/* Button to confirm */}
      <Button variant='contained' onClick={ () => setResult('Awesome')}>
        Calculate savings !
      </Button>
      <ResultDisplay coolText={result}></ResultDisplay>
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
    <Container style={{minHeight: '300px'}}>
      <Typography align='left' variant='h6'>Your Week</Typography>
      <div>some 5 days as a selection</div>
    </Container>
  );
}

const CarSelection = () => {
  return(
    <Container style={{ minHeight: '220px', marginBottom: '20px'}}>
      <Typography align='left' variant='h6'>Your Car</Typography>
      <>CAR GO BROOM</>

    </Container>
  );
}

const ResultDisplay = (props) => {
  return(
    <Container>
      <Typography variant='h1' color='green'>{props.coolText}</Typography>
    </Container>
  );
}



export default App;
