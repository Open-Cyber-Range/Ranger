import './App.css';
import {BrowserRouter as Router, Routes, Route} from 'react-router-dom';
import ExerciseForm from './pages/Exercise';
import MainNavbar from './components/MainNavBar';
import Home from './pages/Home';

function App() {
  return (
    <Router>
      {<MainNavbar />}
      <Routes>
        <Route path='/' element={<Home />} />
        <Route path='/exercises' element={<ExerciseForm />} />
      </Routes>
    </Router>
  );
}

export default App;
