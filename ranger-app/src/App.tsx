import React from 'react';
import {BrowserRouter as Router, Routes, Route} from 'react-router-dom';
import ExerciseDetail from 'src/pages/ExerciseDetail';
import Exercises from 'src/pages/Exercises';
import MainNavbar from 'src/components/MainNavBar';
import Home from 'src/pages/Home';
import {useKeycloak} from '@react-keycloak/web';
import {useSelector} from 'react-redux';
import ParticipantNavBar from './components/ParticipantNavBar';
import DeploymentDetail from './pages/DeploymentDetail';
import ParticipantDeploymentDetail from './pages/participant/DeploymentDetail';
import EmailLog from './pages/EmailLog';
import SendEmail from './pages/Email';
import {selectedRoleSelector} from './slices/userSlice';
import {UserRole} from './models/userRoles';

const App = () => {
  const {keycloak: {authenticated}} = useKeycloak();
  const currentRole = useSelector(selectedRoleSelector);
  if (authenticated && (currentRole === UserRole.MANAGER)) {
    return (
      <Router>
        <MainNavbar/>
        <Routes>
          <Route path='/' element={<Home/>}/>
          <Route path='/exercises' element={<Exercises/>}/>
          <Route path='/exercises/:exerciseId' element={<ExerciseDetail/>}/>
          <Route path='/exercises/:exerciseId/email' element={<SendEmail/>}/>
          <Route
            path='/exercises/:exerciseId/deployments/:deploymentId'
            element={<DeploymentDetail/>}/>
          <Route path='/exercises/:exerciseId/emails' element={<EmailLog/>}/>
        </Routes>
      </Router>
    );
  }

  if (authenticated && (currentRole === UserRole.PARTICIPANT)) {
    return (
      <Router>
        <ParticipantNavBar/>
        <Routes>
          <Route path='/' element={<Home/>}/>
          <Route
            path='/exercises/:exerciseId/deployments/:deploymentId'
            element={<ParticipantDeploymentDetail/>}/>
        </Routes>
      </Router>
    );
  }

  return null;
};

export default App;
