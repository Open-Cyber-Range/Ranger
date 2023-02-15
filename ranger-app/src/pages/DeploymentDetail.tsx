
import React from 'react';
import {useParams} from 'react-router-dom';

import PageHolder from 'src/components/PageHolder';
import type {DeploymentDetailRouteParameters} from 'src/models/routes';
import {useGetTLOsQuery} from 'src/slices/apiSlice';
import styled from 'styled-components';
import {definedOrSkipToken} from 'src/utils';
import DeploymentDetailsGraph from 'src/components/DeploymentDetails/Graph';
import TloTable from 'src/components/DeploymentDetails/TloTable/TloTable';

const Wrapper = styled.div`
  display: flex;
  flex-direction: column;
  text-align: center;

  > div {
    margin-bottom: 2rem;
  }
`;

const DeploymentDetail = () => {
  const {exerciseId, deploymentId}
  = useParams<DeploymentDetailRouteParameters>();

  const {data: tloMap} = useGetTLOsQuery(
    definedOrSkipToken(exerciseId, deploymentId));

  if (exerciseId && deploymentId) {
    return (
      <PageHolder>
        <DeploymentDetailsGraph
          exerciseId={exerciseId}
          deploymentId={deploymentId}
        />
        <TloTable
          exerciseId={exerciseId}
          deploymentId={deploymentId}
          tloMap={tloMap}/>
      </PageHolder>
    );
  }

  return (
    <PageHolder>
      <Wrapper>
        Error: Missing Exercise Id and / or Deployment Id
      </Wrapper>
    </PageHolder>
  );
};

export default DeploymentDetail;
