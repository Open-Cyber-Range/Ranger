import React from 'react';
import type {Deployment} from 'src/models/deployment';
import DeploymentScoreRow from './DeploymentScoresRow';

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
        <DeploymentScoreRow
          key={deployment.id}
          deployment={deployment}/>
      ))}
    </tbody>
  </table>
);

export default DeploymentScoresTable;
