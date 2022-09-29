import React from 'react';
import './App.css';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import "./pages/exercise"
import Navbar from './components/navbar';
import ExerciseForm from './pages/exercise';
import Home from './pages/home';

function App() {
  return (
    <div className="App">
      <Router>
        {<Navbar />}

        <div className="App-header">
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/exercise" element={<ExerciseForm />} />
          </Routes>
        </div >
      </Router>

    </div >
  );
}

export default App;
