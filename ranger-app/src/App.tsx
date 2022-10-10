import './App.css';
import {BrowserRouter as Router, Routes, Route} from 'react-router-dom';
import ExerciseForm from './pages/Exercise';
import MainNavbar from './components/MainNavBar';
import Home from './pages/Home';

function App() {
  return (
    <div className='App'>
      <Router>
        {<MainNavbar />}

        <div className='App-header'>
          <Routes>
            <Route path='/' element={<Home />} />
            <Route path='/exercise' element={<ExerciseForm />} />
          </Routes>
        </div >
      </Router>

    </div >
  );
}

export default App;
