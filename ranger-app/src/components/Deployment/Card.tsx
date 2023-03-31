import type React from 'react';
import {Button, Card, H2} from '@blueprintjs/core';
import type {Deployment} from 'src/models/deployment';
import {
  useDeleteDeploymentMutation,
  useGetDeploymentElementsQuery,
} from 'src/slices/apiSlice';
import {toastSuccess, toastWarning} from 'src/components/Toaster';
import {useTranslation} from 'react-i18next';
import {useNavigate} from 'react-router-dom';
import ScoreTagGroup from 'src/components/Scoring/ScoreTagGroup';
import StatusBar from './ProgressBar';
import InfoTags from './InfoTags';

const DeploymentCard = ({deployment}: {deployment: Deployment}) => {
  const {t} = useTranslation();
  const [deleteDeployment, _deploymentId] = useDeleteDeploymentMutation();
  const navigate = useNavigate();
  const routeChange = () => {
    navigate(`deployments/${deployment.id}`);
  };

  const deleteCurrentDeployment
  = async () => {
    try {
      const response = await deleteDeployment({
        exerciseId: deployment.exerciseId,
        deploymentId: deployment.id}).unwrap();

      if (response === deployment.id) {
        toastSuccess(t('deployments.deleteSuccess', {
          deploymentName: deployment.name,
        }));
      }
    } catch {
      toastWarning(t('deployments.deleteFail'));
    }
  };

  const {data: potentialDeploymentElements} = useGetDeploymentElementsQuery({
    exerciseId: deployment.exerciseId,
    deploymentId: deployment.id,
  });
  const deploymentElements = potentialDeploymentElements ?? [];

  return (
    <Card interactive elevation={2} onClick={routeChange}>
      <div className='flex flex-row justify-between'>
        <H2>{deployment.name}</H2>
        <div className='flex items-end ml-auto mr-8 mt-auto mb-auto'>
          <InfoTags deploymentElements={deploymentElements}/>
        </div>
        <div className='
          flex flex-row justify-end [&>button]:ml-4 [&>button]:mb-2'
        >
          <Button
            large
            intent='danger'
            onClick={async event => {
              event.stopPropagation();
              await deleteCurrentDeployment();
            }}
          > {t('common.delete')}
          </Button>
        </div>
      </div>
      <StatusBar
        key={deployment.id}
        deployment={deployment}
        deploymentElements={deploymentElements}
      />
      <div className='flex items-start mt-4'>
        <ScoreTagGroup
          exerciseId={deployment.exerciseId}
          deploymentId={deployment.id}/>
      </div>
    </Card>
  );
};

export default DeploymentCard;
