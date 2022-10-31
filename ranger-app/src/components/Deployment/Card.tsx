import React from 'react';
import {Button, Card, H2} from '@blueprintjs/core';
import styled from 'styled-components';
import type {Deployment} from 'src/models/Deployment';

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
  }
`;

const DeploymentCard = ({deployment}: {deployment: Deployment}) => (
  <Card interactive elevation={2}>
    <CardRow>
      <H2>{deployment.name}</H2>
      <ActionButtons>
        <Button large intent='danger'> Delete</Button>
      </ActionButtons>
    </CardRow>
  </Card>
);

export default DeploymentCard;
