import React from 'react';
import {
  BrowserRouter as Router,
  Routes,
  Route,
  Navigate,
} from 'react-router-dom';
import ExerciseDetail from 'src/pages/ExerciseDetail';
import Exercises from 'src/pages/Exercises';
import MainNavbar from 'src/components/MainNavBar';
import Home from 'src/pages/Home';
import Logs from 'src/pages/Logs';
import HomeParticipant from 'src/pages/participant/Home';
import {useKeycloak} from '@react-keycloak/web';
import {LogProvider} from 'src/contexts/LogContext';
import ParticipantExercises from './pages/participant/Exercises';
import ParticipantNavBar from './components/ParticipantNavBar';
import DeploymentDetail from './pages/DeploymentDetail';
import ParticipantDeploymentDetail from './pages/participant/DeploymentDetail';
import EmailLog from './pages/EmailLog';
import SendEmail from './pages/Email';
import {UserRole} from './models/userRoles';
import useDefaultRoleSelect from './hooks/useDefaultRoleSelect';
import ScoreDetail from './pages/ScoreDetail';
import DeploymentFocus from './pages/DeploymentFocus';
import MinimalNavBar from './components/MinimalNavBar';
import RolesFallback from './pages/RolesFallback';

const App = () => {
  const {keycloak: {authenticated}} = useKeycloak();
  const currentRole = useDefaultRoleSelect();

  if (authenticated && (currentRole === UserRole.MANAGER)) {
    return (
      <LogProvider>
        <Router>
          <MainNavbar/>
          <Routes>
            <Route path='/' element={<Home/>}/>
            <Route path='/exercises' element={<Exercises/>}/>
            <Route path='/logs' element={<Logs/>}/>
            <Route path='/exercises/:exerciseId' element={<ExerciseDetail/>}/>
            <Route path='/exercises/:exerciseId/email' element={<SendEmail/>}/>
            <Route
              path='/exercises/:exerciseId/deployments/:deploymentId'
              element={<DeploymentDetail/>}/>
            <Route
              path='/exercises/:exerciseId/deployments/:deploymentId/focus'
              element={<DeploymentFocus/>}/>
            <Route
              path='/exercises/:exerciseId/deployments/:deploymentId/scores/:role'
              element={<ScoreDetail/>}/>
            <Route path='/exercises/:exerciseId/emails' element={<EmailLog/>}/>
          </Routes>
        </Router>
      </LogProvider>
    );
  }

  if (authenticated && (currentRole === UserRole.PARTICIPANT)) {
    return (
      <Router>
        <ParticipantNavBar/>
        <Routes>
          <Route path='/' element={<HomeParticipant/>}/>
          <Route path='/exercises' element={<ParticipantExercises/>}/>
          <Route
            path='/exercises/:exerciseId/deployments/:deploymentId'
            element={<ParticipantDeploymentDetail/>}/>
        </Routes>
      </Router>
    );
  }

  if (authenticated && (currentRole === undefined)) {
    return (
      <Router>
        <MinimalNavBar/>
        <Routes>
          <Route path='/' element={<RolesFallback/>}/>
          <Route path='/*' element={<Navigate replace to='/'/>}/>
        </Routes>
      </Router>
    );
  }

  return null;
};

export default App;
