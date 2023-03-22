import type {Deployment} from 'src/models/deployment';
import React from 'react';
import {sortByProperty} from 'sort-by-property';
import DeploymentCard from './Card';

const DeploymentList = ({deployments}: {deployments: Deployment[]}) => {
  deployments = deployments.slice().sort(sortByProperty('updatedAt', 'desc'));

  return (
    <div className='flex flex-col [&>div]:mb-8'>
      {deployments.map(deployment => (
        <div key={deployment.id}>
          <DeploymentCard key={deployment.id} deployment={deployment}/>
        </div>
      ))}
    </div>

  );
};

export default DeploymentList;
