import logo from './logo.svg';
import './App.css';
import { useState } from 'react';
import '@mui/material/Button'

const App = () => {

  return (
    <div className="App">
      <RouteSelection></RouteSelection>
      <CommuteTypeSelection></CommuteTypeSelection>
      {/* TODO: Car selection is only shown when car has been chosen as one of the commute choices */}
      <CarSelection></CarSelection>
      {/* Button to confirm */}
      {/* Result screen */}
      </div>
  );
}

const RouteSelection = () => {
  return(
    <div>
      
    </div>
  );
}

const CommuteTypeSelection = () => {
  return(
    <div>
      
    </div>
  );
}

const CarSelection = () => {
  return(
    <div>

    </div>
  );
}



export default App;
