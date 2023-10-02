import React from 'react';
import {useParams} from 'react-router-dom';
import type {DeploymentDetailRouteParameters} from 'src/models/routes';
import PariticpantSidebar from 'src/components/Exercise/participant/SideBar';
import ParticipantDashboard from 'src/components/Deployment/participant/DashBoard';
import ParticipantScore from 'src/components/Deployment/participant/Score';
import PariticpantEvents from 'src/components/Deployment/participant/Events';
import ManualMetrics from 'src/components/Deployment/participant/ManualMetrics';
import AccountList from 'src/components/Deployment/AccountList';
import {
  useParticipantGetDeploymentScenarioQuery,
  useParticipantGetDeploymentScoresQuery,
  useParticipantGetDeploymentUsersQuery,
  useParticipantGetNodeDeploymentElementsQuery,
  useParticipantGetTriggeredEventsQuery,
} from 'src/slices/apiSlice';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {useSelector} from 'react-redux';
import {selectedEntity} from 'src/slices/userSlice';
import {tryIntoScoringMetadata} from 'src/utils';

const ParticipantDeploymentDetail = () => {
  const {exerciseId, deploymentId} = useParams<DeploymentDetailRouteParameters>();
  const entitySelector = useSelector(selectedEntity);
  const generalQueryArgs = exerciseId && deploymentId ? {exerciseId, deploymentId} : skipToken;
  const participantQueryArgs = exerciseId && deploymentId && entitySelector
    ? {exerciseId, deploymentId, entitySelector} : skipToken;

  const {data: users} = useParticipantGetDeploymentUsersQuery(generalQueryArgs);
  const {data: scores} = useParticipantGetDeploymentScoresQuery(participantQueryArgs);
  const {data: scenario} = useParticipantGetDeploymentScenarioQuery(participantQueryArgs);
  const {data: deplyomentEvents} = useParticipantGetTriggeredEventsQuery(participantQueryArgs);
  const {data: nodeDeploymentElements}
  = useParticipantGetNodeDeploymentElementsQuery(participantQueryArgs);

  if (exerciseId && deploymentId) {
    return (
      <PariticpantSidebar renderMainContent={activeTab => (
        <>
          {activeTab === 'Dash'
            && <ParticipantDashboard/>}
          {activeTab === 'Score'
            && <ParticipantScore
              scoringData={tryIntoScoringMetadata(scenario)}
              scores={scores}/>}
          {activeTab === 'Accounts'
            && <AccountList
              users={users}
              deploymentElements={nodeDeploymentElements}/>}
          {activeTab === 'Events'
            && <PariticpantEvents
              scenarioEvents={scenario?.events}
              deploymentEvents={deplyomentEvents}/>}
          {activeTab === 'Manual Metrics'
            && <ManualMetrics
              exerciseId={exerciseId}
              deploymentId={deploymentId}/>}
        </>
      )}
      />
    );
  }

  return null;
};

export default ParticipantDeploymentDetail;
