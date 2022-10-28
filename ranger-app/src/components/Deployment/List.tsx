import React, {useState, useEffect} from 'react';
import axios from 'axios';
import styled from 'styled-components';
import type {Deployment} from 'src/models/Deployment';
import DeploymentCard from './Card';

const Wrapper = styled.div`
  display: flex;
  flex-direction: column;

  > div {
    margin-bottom: 2rem;
  }
`;
const DeploymentList = ({exerciseId}: {exerciseId: string}) => {
  const [deployments, setDeployments] = useState<Deployment[]>([]);
  useEffect(() => {
    const fetchData = async () =>
      (axios.get<Deployment[]>(`api/v1/exercise/${exerciseId}/deployment`));
    fetchData().then(response => {
      setDeployments(response.data);
    }).catch(_error => {
      throw new Error('Error retrieving deployment data');
    });
  }, [exerciseId]);

  return (
    <Wrapper>
      {deployments.map(deployment => (
        <DeploymentCard key={deployment.id} deployment={deployment}/>
      ))}

    </Wrapper>

  );
};

export default DeploymentList;
