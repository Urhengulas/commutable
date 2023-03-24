import logo from './logo.svg';
import './App.css';
import { useState } from 'react';
import '@mui/material/Button'
import  { Container, Button, Typography, TextField, ThemeProvider } from '@mui/material';
import axios, {isCancel, AxiosError} from 'axios';
import { border } from '@mui/system';


const App = () => {
  const [result, setResult] = useState('Shitty');
  const [home, setHome] = useState('');
  const [work, setWork] = useState('');
  const [requestDone, setRequestDone] = useState(false)
  const [data, setData] = useState({})

  return (
    <Container className="App" maxWidth='xs' style={{ boxShadow: '10px 10px 52px 0px rgba(0,0,0,0.39)', paddingBottom: '30px', minHeight: '100vh'}}>
      <Title />
      <RouteSelection setHome={setHome} setWork={setWork} />
      {/* TODO: Car selection is only shown when car has been chosen as one of the commute choices */}
      <CarSelection />
      {/* Button to confirm */}
      <Button variant='contained' onClick={ () => {
        console.log(sendRequest(setData, setRequestDone ,"CAR", home, work, "gas", "medium"));
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

const sendRequest = (setData, setDone, type, origin, destination, propulsionType, carSize) => {
  let uriString = 'http://localhost:3030/car?origin=' + origin + '&destination=' + destination + '&propulsion=gas&size=medium';
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
