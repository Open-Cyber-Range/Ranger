import React, {useEffect} from 'react';
import {useNavigate, useParams} from 'react-router-dom';
import type {DeploymentDetailRouteParameters} from 'src/models/routes';
import {useTranslation} from 'react-i18next';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {
  useAdminDeleteDeploymentMutation,
  useAdminGetDeploymentElementsQuery,
  useAdminGetDeploymentQuery,
  useAdminGetDeploymentScenarioQuery,
  useAdminGetDeploymentScoresQuery,
} from 'src/slices/apiSlice';
import DeploymentDetailsGraph from 'src/components/Scoring/Graph';
import Editor from '@monaco-editor/react';
import {
  AnchorButton,
  Callout,
  Card,
  Elevation,
  H2,
} from '@blueprintjs/core';
import SideBar from 'src/components/Exercise/SideBar';
import useExerciseStreaming from 'src/hooks/useExerciseStreaming';
import {toastSuccess, toastWarning} from 'src/components/Toaster';
import RoleScoresButtonGroup from 'src/components/Scoring/RoleScoresButtonGroup';
import {tryIntoScoringMetadata, isVMDeploymentOngoing} from 'src/utils';
import {Tooltip2} from '@blueprintjs/popover2';
import InfoTags from 'src/components/Deployment/InfoTags';

const DeploymentDetail = () => {
  const {t} = useTranslation();
  const navigate = useNavigate();
  const {exerciseId, deploymentId} = useParams<DeploymentDetailRouteParameters>();
  useExerciseStreaming(exerciseId);
  const queryArguments = exerciseId && deploymentId ? {exerciseId, deploymentId} : skipToken;
  const {data: deployment} = useAdminGetDeploymentQuery(queryArguments);
  const {data: scenario} = useAdminGetDeploymentScenarioQuery(queryArguments);
  const {data: scores} = useAdminGetDeploymentScoresQuery(queryArguments);
  const {data: deploymentElements} = useAdminGetDeploymentElementsQuery(queryArguments);
  const [deleteDeployment] = useAdminDeleteDeploymentMutation();

  const [deplyomentInProgress, setDeplyomentInProgress] = React.useState(false);
  const [deplyomentBeingRemoved, setDeplyomentBeingRemoved] = React.useState(false);

  useEffect(() => {
    const isOngoing = isVMDeploymentOngoing(deploymentElements ?? []);
    setDeplyomentInProgress(isOngoing);
  }, [deploymentElements]);

  const handleDeleteDeployment = async () => {
    try {
      setDeplyomentBeingRemoved(true);
      const response = await deleteDeployment({
        exerciseId: deployment?.exerciseId ?? '',
        deploymentId: deployment?.id ?? '',
      }).unwrap();

      if (response === deployment?.id) {
        setDeplyomentBeingRemoved(false);
        toastSuccess(t('deployments.deleteSuccess', {
          deploymentName: deployment?.name,
        }));
        navigate(`/exercises/${deployment.exerciseId}`);
      }
    } catch {
      setDeplyomentBeingRemoved(false);
      toastWarning(t('deployments.deleteFail'));
    }
  };

  if (exerciseId && deploymentId && deployment && scenario) {
    return (
      <SideBar renderMainContent={() => (
        <>
          <div className='flex justify-between overflow-auto'>
            <div className='flex space-x-6'>
              <H2>{deployment?.name}</H2>
              <InfoTags deploymentElements={deploymentElements ?? []}/>
            </div>
            <Tooltip2
              content={deplyomentInProgress
                ? t('deployments.beingDeployed') ?? ''
                : (deplyomentBeingRemoved
                  ? t('deployments.beingDeleted') ?? ''
                  : '')}
              disabled={!deplyomentInProgress && !deplyomentBeingRemoved}
            >
              <AnchorButton
                icon='trash'
                intent='danger'
                disabled={deplyomentInProgress || deplyomentBeingRemoved}
                loading={deplyomentBeingRemoved}
                onClick={handleDeleteDeployment}
              >
                {t('common.delete')}
              </AnchorButton>
            </Tooltip2>
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
    <Callout title={t('exercises.noDeploymentInfo') ?? ''}/>
  );
};

export default DeploymentDetail;
