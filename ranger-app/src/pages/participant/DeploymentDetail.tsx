import React from 'react';
import {useParams} from 'react-router-dom';
import type {DeploymentDetailRouteParameters} from 'src/models/routes';
import PariticpantSidebar from 'src/components/Exercise/participant/SideBar';
import ParticipantDashboard from 'src/components/Deployment/participant/DashBoard';
import Accounts from 'src/components/Deployment/participant/Accounts';
import ParticipantScore from 'src/components/Deployment/participant/Score';
import Events from 'src/components/Deployment/participant/Events';
import ManualMetrics from 'src/components/Deployment/participant/ManualMetrics';

const ParticipantDeploymentDetail = () => {
  const {exerciseId, deploymentId} = useParams<DeploymentDetailRouteParameters>();

  if (exerciseId && deploymentId) {
    return (
      <PariticpantSidebar renderMainContent={activeTab => (
        <>
          {activeTab === 'Dash'
            && <ParticipantDashboard
              exerciseId={exerciseId}
              deploymentId={deploymentId}/>}
          {activeTab === 'Score'
            && <ParticipantScore
              exerciseId={exerciseId}
              deploymentId={deploymentId}/>}
          {activeTab === 'Accounts'
            && <Accounts
              exerciseId={exerciseId}
              deploymentId={deploymentId}/>}
          {activeTab === 'Events'
            && <Events
              exerciseId={exerciseId}
              deploymentId={deploymentId}/>}
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
