import styled from 'styled-components';
import type {Deployment} from 'src/models/deployment';
import React from 'react';
import {sortByUpdatedAtDescending} from 'src/utils';
import DeploymentCard from './Card';

const Wrapper = styled.div`
  display: flex;
  flex-direction: column;

  > div {
    margin-bottom: 2rem;
  }
`;

const DeploymentList = ({deployments}: {deployments: Deployment[]}) => {
  deployments = deployments.slice().sort(sortByUpdatedAtDescending);

  return (
    <Wrapper>
      {deployments.map(deployment => (
        <DeploymentCard key={deployment.id} deployment={deployment}/>
      ))}

    </Wrapper>

  );
};

export default DeploymentList;
