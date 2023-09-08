import React from 'react';
import {useParams} from 'react-router-dom';
import type {DeploymentDetailRouteParameters} from 'src/models/routes';
import {useTranslation} from 'react-i18next';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {
  useAdminDeleteDeploymentMutation,
  useAdminGetDeploymentQuery,
  useAdminGetDeploymentScenarioQuery,
  useAdminGetDeploymentScoresQuery,
} from 'src/slices/apiSlice';
import DeploymentDetailsGraph from 'src/components/Scoring/Graph';
import Editor from '@monaco-editor/react';
import {AnchorButton, Card, Elevation, H2} from '@blueprintjs/core';
import SideBar from 'src/components/Exercise/SideBar';
import useExerciseStreaming from 'src/hooks/useExerciseStreaming';
import {toastSuccess, toastWarning} from 'src/components/Toaster';
import RoleScoresButtonGroup from 'src/components/Scoring/RoleScoresButtonGroup';
import {tryIntoScoringMetadata} from 'src/utils';

const DeploymentDetail = () => {
  const {t} = useTranslation();
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
          <div className='pt-8 pb-4'>
            <RoleScoresButtonGroup
              exerciseId={exerciseId}
              deploymentId={deploymentId}
              scenario={scenario}
              scores={scores ?? []}
            />
          </div>
          <DeploymentDetailsGraph
            colorsByRole
            scoringData={tryIntoScoringMetadata(scenario)}
            scores={scores ?? []}
          />
          <Card className='h-[40vh] p-0' elevation={Elevation.TWO}>
            <Editor
              value={deployment?.sdlSchema}
              defaultLanguage='yaml'
              options={{readOnly: true}}
            />
          </Card>
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
