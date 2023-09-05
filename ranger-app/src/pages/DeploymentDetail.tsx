import React from 'react';
import {useNavigate, useParams} from 'react-router-dom';
import type {DeploymentDetailRouteParameters} from 'src/models/routes';
import {useTranslation} from 'react-i18next';
import BackButton from 'src/components/BackButton';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {
  useAdminDeleteDeploymentMutation,
  useAdminGetDeploymentQuery,
  useAdminGetDeploymentScenarioQuery,
  useAdminGetDeploymentScoresQuery,
} from 'src/slices/apiSlice';
import DeploymentDetailsGraph from 'src/components/Scoring/Graph';
import Editor from '@monaco-editor/react';
import {AnchorButton, H2} from '@blueprintjs/core';
import SideBar from 'src/components/Exercise/SideBar';
import useExerciseStreaming from 'src/hooks/useExerciseStreaming';
import {toastSuccess, toastWarning} from 'src/components/Toaster';
import AccountList from 'src/components/Deployment/AccountList';
import EntityConnector from 'src/components/Deployment/EntityConnector';
import EntityTree from 'src/components/Deployment/EntityTree';
import MetricScorer from 'src/components/Scoring/MetricScorer';
import RoleScoresButtonGroup from 'src/components/Scoring/RoleScoresButtonGroup';

const DeploymentDetail = () => {
  const {t} = useTranslation();
  const navigate = useNavigate();
  const {exerciseId, deploymentId} = useParams<DeploymentDetailRouteParameters>();
  useExerciseStreaming(exerciseId);
  const queryArguments = exerciseId && deploymentId ? {exerciseId, deploymentId} : skipToken;
  const {data: deployment} = useAdminGetDeploymentQuery(queryArguments);
  const {data: scenario} = useAdminGetDeploymentScenarioQuery(queryArguments);
  const {data: scores} = useAdminGetDeploymentScoresQuery(queryArguments);

  const [deleteDeployment] = useAdminDeleteDeploymentMutation();

  const handleDeleteDeployment = async () => {
    try {
      const response = await deleteDeployment({
        exerciseId: deployment?.exerciseId ?? '',
        deploymentId: deployment?.id ?? '',
      }).unwrap();

      if (response === deployment?.id) {
        toastSuccess(t('deployments.deleteSuccess', {
          deploymentName: deployment?.name,
        }));
        navigate(`/exercises/${deployment.exerciseId}`);
      }
    } catch {
      toastWarning(t('deployments.deleteFail'));
    }
  };

  if (exerciseId && deploymentId && deployment && scenario) {
    return (
      <SideBar renderMainContent={() => (
        <>
          <div className='flex'>
            <H2>{deployment?.name}</H2>
            <span className='ml-auto'>
              <AnchorButton
                icon='trash'
                intent='danger'
                onClick={handleDeleteDeployment}
              >
                {t('common.delete')}
              </AnchorButton>
            </span>
          </div>
          <br/>
          <div className='h-[40vh]'>
            <Editor
              value={deployment?.sdlSchema}
              defaultLanguage='yaml'
              options={{readOnly: true}}
            />
          </div>
          <DeploymentDetailsGraph
            colorsByRole
            entities={scenario.entities ?? {}}
            tlos={scenario.tlos ?? {}}
            evaluations={scenario.evaluations ?? {}}
            metrics={scenario.metrics ?? {}}
            scenarioStart={scenario?.start ?? ''}
            scenarioEnd={scenario?.end ?? ''}
            scores={scores ?? []}
          />
          <RoleScoresButtonGroup
            exerciseId={exerciseId}
            deploymentId={deploymentId}
            scenario={scenario}
            scores={scores ?? []}
          />
          <AccountList
            exerciseId={exerciseId}
            deploymentId={deploymentId}
          />
          <div className='mt-[2rem]'>
            <EntityConnector exerciseId={exerciseId} deploymentId={deploymentId}/>
          </div>
          <div className='mt-[2rem]'>
            <EntityTree exerciseId={exerciseId} deploymentId={deploymentId}/>
          </div>
          <div className='flex justify-end items-center pb-4 mt-[2rem]'>
            <div className='flex justify-between items-center'>
              <BackButton/>
            </div>
          </div>
          <MetricScorer
            exerciseId={exerciseId}
            deploymentId={deploymentId}/>
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

export default DeploymentDetail;
