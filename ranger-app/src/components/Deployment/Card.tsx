import React from 'react';
import {Button, Card, H2} from '@blueprintjs/core';
import styled from 'styled-components';
import type {Deployment} from 'src/models/deployment';
import {
  useDeleteDeploymentMutation,
  useGetDeploymentElementsQuery,
} from 'src/slices/apiSlice';
import {toastSuccess, toastWarning} from 'src/components/Toaster';
import StatusBar from './ProgressBar';
import Tags from './Tags';

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

const Status = styled.div`
  display: flex;
  align-items: end;
  margin-left: auto;
  margin-right: 2rem;
`;

const DeploymentCard = ({deployment}: {deployment: Deployment}) => {
  const [deleteDeployment, _deploymentId] = useDeleteDeploymentMutation();
  const deleteCurrentDeployment = async () => {
    try {
      const response = await deleteDeployment({
        exerciseId: deployment.exerciseId,
        deploymentId: deployment.id}).unwrap();

      if (response === deployment.id) {
        toastSuccess(`Deployment "${deployment.name}" deleted`);
      }
    } catch {
      toastWarning('Failed to delete the deployment');
    }
  };

  const {data: potentialDeploymentElements} = useGetDeploymentElementsQuery({
    exerciseId: deployment.exerciseId,
    deploymentId: deployment.id,
  });
  const deploymentElements = potentialDeploymentElements ?? [];

  return (
    <Card interactive elevation={2}>
      <CardRow>
        <H2>{deployment.name}</H2>
        <Status>
          <Tags deploymentElements={deploymentElements}/>
        </Status>

        <ActionButtons>
          <Button
            large
            intent='danger'
            onClick={async () => {
              await deleteCurrentDeployment();
            }}
          > Delete
          </Button>
        </ActionButtons>
      </CardRow>
      <StatusBar
        key={deployment.id}
        deployment={deployment}
        deploymentElements={deploymentElements}
      />
    </Card>
  );
};

export default DeploymentCard;
