
import React from 'react';
import {useParams} from 'react-router-dom';
import PageHolder from 'src/components/PageHolder';
import type {DeploymentDetailRouteParameters} from 'src/models/routes';
import {useGetTLOsQuery} from 'src/slices/apiSlice';
import DeploymentDetailsGraph from 'src/components/DeploymentDetails/Graph';
import TloTable from 'src/components/DeploymentDetails/TloTable/TloTable';
import {useTranslation} from 'react-i18next';
import BackButton from 'src/components/BackButton';
import {skipToken} from '@reduxjs/toolkit/dist/query';

const DeploymentDetail = () => {
  const {t} = useTranslation();
  const {exerciseId, deploymentId}
  = useParams<DeploymentDetailRouteParameters>();
  const {data: tloMap} = useGetTLOsQuery(
    exerciseId && deploymentId ? {exerciseId, deploymentId} : skipToken);

  if (exerciseId && deploymentId) {
    return (
      <PageHolder>
        <BackButton/>
        <DeploymentDetailsGraph
          exerciseId={exerciseId}
          deploymentId={deploymentId}
        />
        <TloTable
          exerciseId={exerciseId}
          deploymentId={deploymentId}
          tloMap={tloMap}/>
      </PageHolder>
    );
  }

  return (
    <div className='
    flex justify-center align-center m-2 mt-10 mb-auto text-gray-400'
    >
      {t('exercises.noDeploymentInfo')}
    </div>
  );
};

export default DeploymentDetail;
