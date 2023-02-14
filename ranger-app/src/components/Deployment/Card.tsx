import type React from 'react';
import {Button, Card, H2} from '@blueprintjs/core';
import styled from 'styled-components';
import type {Deployment} from 'src/models/deployment';
import {
  useDeleteDeploymentMutation,
  useGetDeploymentElementsQuery,
} from 'src/slices/apiSlice';
import {toastSuccess, toastWarning} from 'src/components/Toaster';
import {useTranslation} from 'react-i18next';
import {useNavigate} from 'react-router-dom';
import StatusBar from './ProgressBar';
import InfoTags from './InfoTags';
import ScoreTags from './ScoreTags/ScoreTags';

const CardRow = styled.div`
  display: flex;
  flex-direction: row;
  justify-content: space-between;
`;

const ActionButtons = styled.div`
  display: flex;
  flex-direction: row;
  justify-content: flex-end;
  > button {
    margin-left: 1rem;
    margin-bottom: 0.5rem;
    align-self: center;
  }
`;

const TagsWrapper = styled.div`
  display: flex;
  align-items: end;
  margin-left: auto;
  margin-right: 2rem;
`;

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
      <CardRow>
        <H2>{deployment.name}</H2>
        <TagsWrapper>
          <InfoTags deploymentElements={deploymentElements}/>
        </TagsWrapper>
        <ActionButtons>
          <Button
            large
            intent='danger'
            onClick={async event => {
              event.stopPropagation();
              await deleteCurrentDeployment();
            }}
          > {t('common.delete')}
          </Button>
        </ActionButtons>
      </CardRow>
      <StatusBar
        key={deployment.id}
        deployment={deployment}
        deploymentElements={deploymentElements}
      />
      <ScoreTags
        exerciseId={deployment.exerciseId}
        deploymentId={deployment.id}/>
    </Card>
  );
};

export default DeploymentCard;
