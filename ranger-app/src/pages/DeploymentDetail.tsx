import React from 'react';
import {useParams} from 'react-router-dom';
import type {DeploymentDetailRouteParameters} from 'src/models/routes';
import {useTranslation} from 'react-i18next';
import BackButton from 'src/components/BackButton';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {
  useAdminDeleteDeploymentMutation,
  useAdminGetDeploymentQuery,
  useAdminGetDeploymentScenarioQuery,
} from 'src/slices/apiSlice';
import DeploymentDetailsGraph from 'src/components/Scoring/Graph';
import TloTable from 'src/components/Scoring/TloTable';
import Editor from '@monaco-editor/react';
import {AnchorButton, H2} from '@blueprintjs/core';
import SideBar from 'src/components/Exercise/SideBar';
import useExerciseStreaming from 'src/hooks/useExerciseStreaming';
import {toastSuccess, toastWarning} from 'src/components/Toaster';

const DeploymentDetail = () => {
  const {t} = useTranslation();
  const {exerciseId, deploymentId}
  = useParams<DeploymentDetailRouteParameters>();
  useExerciseStreaming(exerciseId);
  const {data: scenario} = useAdminGetDeploymentScenarioQuery(
    exerciseId && deploymentId ? {exerciseId, deploymentId} : skipToken,
  );
  const {data: deployment} = useAdminGetDeploymentQuery(
    exerciseId && deploymentId ? {exerciseId, deploymentId} : skipToken,
  );

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

  if (exerciseId && deploymentId) {
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
            exerciseId={exerciseId}
            deploymentId={deploymentId}
          />
          <TloTable
            exerciseId={exerciseId}
            deploymentId={deploymentId}
            tloMap={scenario?.tlos}
          />
          <div className='flex justify-between items-center pb-4'>
            <BackButton/>
          </div>
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
