import logo from './logo.svg';
import './App.css';
import { useState } from 'react';
import '@mui/material/Button'
import  { Container, Button, Typography, TextField, ToggleButton, ToggleButtonGroup } from '@mui/material';
import axios, {isCancel, AxiosError} from 'axios';
import { border } from '@mui/system';


const App = () => {
  const [result, setResult] = useState('Shitty');
  const [home, setHome] = useState('');
  const [work, setWork] = useState('');
  const [requestDone, setRequestDone] = useState(false)
  const [data, setData] = useState({})
  const [fuelType, setFuelType] = useState('gas');
  const [carSize, setCarSize] = useState('medium');


  const handleFueltypeChange = (
    event,
    newAlignment,
  ) => {
    setFuelType(newAlignment);
    console.log(fuelType)
  };

  const handleCarSizeChange = (
    event,
    newAlignment,
  ) => {
    setCarSize(newAlignment);
    console.log(carSize)
  };

  return (
    <Container className="App" maxWidth='xs' style={{ boxShadow: '10px 10px 52px 0px rgba(0,0,0,0.39)', paddingBottom: '30px', minHeight: '100vh'}}>
      <Title />
      <RouteSelection setHome={setHome} setWork={setWork} />
      {/* TODO: Car selection is only shown when car has been chosen as one of the commute choices */}
      <CarSelection handleFuelTypeChange={handleFueltypeChange} handleCarSizeChange={handleCarSizeChange} fuelType={fuelType} carSize={carSize} />
      {/* Button to confirm */}
      <Button variant='contained' onClick={ () => {
        setRequestDone(false)
        console.log(sendRequest(setData, setRequestDone ,"CAR", home, work, fuelType, carSize));
       }}>
        Calculate savings !
      </Button>
        <ResultDisplay data={data} done={requestDone} />
      </Container>
  );
}


const Title = () => {
  return(
    <Container margin='normal'>
      <Typography align='left' variant='h3' style={{paddingTop: '30px', paddingBottom: '30px'}}>GreenCommute</Typography>
    </Container>
  );
}



const RouteSelection = (props) => {
  return(
    <Container margin='normal'>
      <Typography align='left' variant='h6'>Your Addresses</Typography>
      <TextField margin='normal' fullWidth='90%'
        id="homeAddress"
        label="Home Address"
        onChange={ (e) => {
            props.setHome(e.target.value)
          }
        }
      />
      <br />
      <TextField margin='normal' fullWidth='90%'
      id="workAddress"
      label="Work Address"
      onChange={ (e) => {
        props.setWork(e.target.value)
      }
    }
    />
    </Container>
  );
}

const CarSelection = (props) => {
  const handleChange = (e) => {
    console.log(e.target.value)
    props.setFuelType(e.target.value)
  }

  return(
    <Container style={{ minHeight: '220px', marginBottom: '20px'}}>
      <Typography align='left' variant='h6'>Your Car</Typography>
      <br />
      <Typography variant="overline">Fuel type</Typography>
      <br />
      <ToggleButtonGroup
      color="primary"
      value={props.fuelType}
      exclusive
      onChange={props.handleFuelTypeChange}
      aria-label="FuelType"
      >
        <ToggleButton value="gas">Gas</ToggleButton>
        <ToggleButton value="diesel">Diesel</ToggleButton>
        <ToggleButton value="electric">Electric</ToggleButton>
      </ToggleButtonGroup>
      <br />
      <br />
      <Typography variant="overline" >Car size </Typography>
      <br />
      <ToggleButtonGroup
      color="primary"
      value={props.carSize}
      exclusive
      onChange={props.handleCarSizeChange}
      aria-label="CarSize"
      >
        <ToggleButton value="small">Small</ToggleButton>
        <ToggleButton value="medium">Medium</ToggleButton>
        <ToggleButton value="large">Large</ToggleButton>
      </ToggleButtonGroup>
      
    </Container>
  );
}

const ResultDisplay = (props) => {
  if(!props.done) {
    return (<div></div>);
  }

  return(
    <Container style={{ paddingTop: '30px', paddingBottom: '30px' }}>
      <Typography variant='h4' color='green'>{(props.data.emissions/ 1000 ) + "kg CO2"}</Typography>
      <Typography variant='h5'>{ Math.round(props.data.duration/ 60) } minutes traveltime</Typography>
    </Container>
  );
}

//#region Non-Component functions

const sendRequest = (setData, setDone, type, origin, destination, fuelType, carSize) => {
  let uriString = 'http://localhost:3030/car?origin=' + origin + '&destination=' + destination + '&propulsion=' + fuelType + '&size=' + carSize;
  let encoded = encodeURI(uriString);
  console.log(encoded);

  axios.get(encoded)
  .then((response) => {
    console.log(response)
    setData(response.data);
    setDone(true);
  });
}

//#endregion



export default App;
