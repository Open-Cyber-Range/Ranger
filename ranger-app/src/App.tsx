import React from 'react';
import './App.css';
import {BrowserRouter as Router, Routes, Route} from 'react-router-dom';
import ExerciseDetail from 'src/pages/ExerciseDetail';
import Exercises from 'src/pages/Exercises';
import MainNavbar from 'src/components/MainNavBar';
import Home from 'src/pages/Home';

const App = () => (

  <Router>
    <MainNavbar/>
    <Routes>
      <Route path='/' element={<Home/>}/>
      <Route path='/exercises' element={<Exercises/>}/>
      <Route path='/exercises/:exerciseId' element={<ExerciseDetail/>}/>
    </Routes>
  </Router>
);

export default App;
