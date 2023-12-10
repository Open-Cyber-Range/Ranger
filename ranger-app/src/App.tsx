import React, {useEffect} from 'react';
import {
  BrowserRouter as Router,
  Routes,
  Route,
  Navigate,
} from 'react-router-dom';
import ExerciseDetail from 'src/pages/ExerciseDetail';
import Exercises from 'src/pages/Exercises';
import Home from 'src/pages/Home';
import Logs from 'src/pages/Logs';
import HomeParticipant from 'src/pages/participant/Home';
import {useKeycloak} from '@react-keycloak/web';
import {LogProvider} from 'src/contexts/LogContext';
import {useDispatch} from 'react-redux';
import ParticipantExercises from './pages/participant/Exercises';
import DeploymentDetail from './pages/DeploymentDetail';
import ParticipantDeploymentDetail from './pages/participant/DeploymentDetail';
import {UserRole} from './models/userRoles';
import useDefaultRoleSelect from './hooks/useDefaultRoleSelect';
import ScoreDetail from './pages/ScoreDetail';
import DeploymentFocus from './pages/DeploymentFocus';
import RolesFallback from './pages/RolesFallback';
import {setToken} from './slices/userSlice';
import MainNavbar from './components/Navbar/MainNavBar';
import ManagerNavbarLinks from './components/Navbar/ManagerLinks';
import ParticipantNavbarLinks from './components/Navbar/ParticipantLinks';

const App = () => {
  const {keycloak, keycloak: {authenticated, token}} = useKeycloak();
  const dispatch = useDispatch();

  useEffect(() => {
    if (token !== undefined) {
      dispatch(setToken(token));
    }
  }, [token, dispatch]);

  useEffect(() => {
    keycloak.onTokenExpired = () => {
      keycloak.updateToken(180).then(refreshed => {
        if (refreshed && keycloak.token) {
          dispatch(setToken(keycloak.token));
        }
      }).catch(() => {
        keycloak.clearToken();
      });
    };

    keycloak.onAuthRefreshError = async () => {
      keycloak.clearToken();
    };
  }, [
    keycloak,
    keycloak.token,
    dispatch,
  ]);

  const currentRole = useDefaultRoleSelect();

  if (authenticated && (currentRole === UserRole.MANAGER)) {
    return (
      <LogProvider>
        <Router>
          <MainNavbar navbarLinks={<ManagerNavbarLinks/>}/>
          <Routes>
            <Route path='/' element={<Home/>}/>
            <Route path='/exercises' element={<Exercises/>}/>
            <Route path='/logs' element={<Logs/>}/>
            <Route path='/exercises/:exerciseId' element={<ExerciseDetail/>}/>
            <Route
              path='/exercises/:exerciseId/deployments/:deploymentId'
              element={<DeploymentDetail/>}/>
            <Route
              path='/exercises/:exerciseId/deployments/:deploymentId/focus'
              element={<DeploymentFocus/>}/>
            <Route
              path='/exercises/:exerciseId/deployments/:deploymentId/scores/:role'
              element={<ScoreDetail/>}/>
          </Routes>
        </Router>
      </LogProvider>
    );
  }

  if (authenticated && (currentRole === UserRole.PARTICIPANT)) {
    return (
      <Router>
        <MainNavbar navbarLinks={<ParticipantNavbarLinks/>}/>
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
        <MainNavbar/>
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
