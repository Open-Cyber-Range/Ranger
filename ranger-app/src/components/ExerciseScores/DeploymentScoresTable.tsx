import React from 'react';
import ScoreTags from 'src/components/Deployment/ScoreTags/ScoreTags';
import type {Deployment} from 'src/models/deployment';
import styled from 'styled-components';

const DataCellWrapper = styled.td`
  display: flex;
  flex-direction: row;
  justify-content: space-between;
`;

const DeploymentScoresTable = ({deployments}:
{deployments: Deployment[]}) => (
  <table className='
    bp4-html-table
    bp4-compact
    bp4-html-table-striped
    '
  >
    <tbody>
      {deployments.map(deployment => (
        <tr key={deployment.id}>
          <DataCellWrapper>
            <h2>{deployment.name}</h2>
            <ScoreTags
              exerciseId={deployment.exerciseId}
              deploymentId={deployment.id}/>
          </DataCellWrapper>
        </tr>
      ))}
    </tbody>
  </table>
);

export default DeploymentScoresTable;
