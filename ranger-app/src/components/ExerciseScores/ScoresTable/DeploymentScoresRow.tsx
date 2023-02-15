import React from 'react';
import type {Deployment} from 'src/models/deployment';
import styled from 'styled-components';
import ScoreTags from 'src/components/Deployment/ScoreTags/ScoreTags';

const DataCellWrapper = styled.td`
  display: flex;
  flex-direction: row;
  justify-content: space-between;
`;

const DeploymentScoreRow = ({deployment}:
{deployment: Deployment}) => (
  <tr>
    <DataCellWrapper>
      <h2>{deployment.name}</h2>
      <ScoreTags
        exerciseId={deployment.exerciseId}
        deploymentId={deployment.id}/>
    </DataCellWrapper>
  </tr>
);

export default DeploymentScoreRow;
