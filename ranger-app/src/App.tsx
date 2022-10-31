import React, { Suspense } from 'react';
import './App.css';
import {BrowserRouter as Router, Routes, Route} from 'react-router-dom';
import ExerciseDetail from 'src/pages/ExerciseDetail';
import Exercises from 'src/pages/Exercises';
import MainNavbar from 'src/components/MainNavBar';
import Home from 'src/pages/Home';

const App = () => (
  <Suspense fallback="Loading...">
    <Router>
      <MainNavbar/>
      <Routes>
        <Route path='/' element={<Home/>}/>
        <Route path='/exercises' element={<Exercises/>}/>
        <Route path='/exercises/:exerciseId' element={<ExerciseDetail/>}/>
      </Routes>
    </Router>
  </Suspense>
);

export default App;
