import React from 'react';
import {useParams} from 'react-router-dom';
import PageHolder from 'src/components/PageHolder';
import type {DeploymentDetailRouteParameters} from 'src/models/routes';
import {useTranslation} from 'react-i18next';
import BackButton from 'src/components/BackButton';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {
  useAdminGetDeploymentQuery,
  useAdminGetDeploymentScenarioQuery,
} from 'src/slices/apiSlice';
import DeploymentDetailsGraph from 'src/components/Scoring/Graph';
import TloTable from 'src/components/Scoring/TloTable';
import Editor from '@monaco-editor/react';
import {H2} from '@blueprintjs/core';

const DeploymentDetail = () => {
  const {t} = useTranslation();
  const {exerciseId, deploymentId}
  = useParams<DeploymentDetailRouteParameters>();
  const {data: scenario} = useAdminGetDeploymentScenarioQuery(
    exerciseId && deploymentId ? {exerciseId, deploymentId} : skipToken);
  const {data: deployment} = useAdminGetDeploymentQuery(
    exerciseId && deploymentId ? {exerciseId, deploymentId} : skipToken);

  if (exerciseId && deploymentId) {
    return (
      <PageHolder>
        <H2>{deployment?.name}</H2>
        <br/>
        <div className='h-[40vh]'>
          <Editor
            value={deployment?.sdlSchema}
            defaultLanguage='yaml'
            options={{readOnly: true}}
          />
        </div>
        <DeploymentDetailsGraph
          exerciseId={exerciseId}
          deploymentId={deploymentId}
        />
        <TloTable
          exerciseId={exerciseId}
          deploymentId={deploymentId}
          tloMap={scenario?.tlos}/>
        <BackButton/>
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
