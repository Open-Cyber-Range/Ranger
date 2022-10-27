import React from 'react';
import './App.css';
import {BrowserRouter as Router, Routes, Route} from 'react-router-dom';
import DeploymentForm from 'src/pages/Deployments';
import ExerciseForm from 'src/pages/Exercise';
import MainNavbar from 'src/components/MainNavBar';
import Home from 'src/pages/Home';

const App = () => (
  <Router>
    <MainNavbar/>
    <Routes>
      <Route path='/' element={<Home/>}/>
      <Route path='/exercises' element={<ExerciseForm/>}/>
      <Route path='/exercises/:exerciseName' element={<DeploymentForm/>}/>
    </Routes>
  </Router>
);

export default App;
