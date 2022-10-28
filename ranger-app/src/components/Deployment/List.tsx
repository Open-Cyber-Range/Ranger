import styled from 'styled-components';
import type {Deployment} from 'src/models/Deployment';
import React from 'react';
import DeploymentCard from './Card';

const Wrapper = styled.div`
  display: flex;
  flex-direction: column;

  > div {
    margin-bottom: 2rem;
  }
`;
const DeploymentList = ({deployments}: {deployments: Deployment[]}) => (
  <Wrapper>
    {deployments.map(deployment => (
      <DeploymentCard key={deployment.id} deployment={deployment}/>
    ))}

  </Wrapper>

);

export default DeploymentList;
