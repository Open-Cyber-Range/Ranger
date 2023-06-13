import React from 'react';
import {useParams} from 'react-router-dom';
import type {DeploymentDetailRouteParameters} from 'src/models/routes';
import {useTranslation} from 'react-i18next';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {useParticipantGetDeploymentQuery} from 'src/slices/apiSlice';
import SideBar from 'src/components/Exercise/SideBar';
import ParticipantDeploymnentGraph from 'src/components/Scoring/participant/Graph';
import {H2} from '@blueprintjs/core';

const ParticipantDeploymentDetail = () => {
  const {t} = useTranslation();
  const {exerciseId, deploymentId} = useParams<DeploymentDetailRouteParameters>();
  const {data: deployment} = useParticipantGetDeploymentQuery(
    exerciseId && deploymentId ? {exerciseId, deploymentId} : skipToken,
  );

  if (exerciseId && deploymentId) {
    return (
      <SideBar renderMainContent={() => (
        <>
          <H2>{deployment?.name}</H2>
          <br/>
          <ParticipantDeploymnentGraph
            exerciseId={exerciseId}
            deploymentId={deploymentId}
          />
        </>
      )}
      />
    );
  }

  return (
    <div className='flex justify-center align-center m-2 mt-10 mb-auto text-gray-400'>
      {t('exercises.noDeploymentInfo')}
    </div>
  );
};

export default ParticipantDeploymentDetail;
