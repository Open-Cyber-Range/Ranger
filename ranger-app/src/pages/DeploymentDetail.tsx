
import React from 'react';
import {useParams} from 'react-router-dom';

import PageHolder from 'src/components/PageHolder';
import type {DeploymentDetailRouteParameters} from 'src/models/routes';
import {useGetTLOsQuery} from 'src/slices/apiSlice';
import styled from 'styled-components';
import {definedOrSkipToken} from 'src/utils';
import DeploymentDetailsGraph from 'src/components/DeploymentDetails/Graph';
import TloTable from 'src/components/DeploymentDetails/TloTable/TloTable';
import {useTranslation} from 'react-i18next';
import BackButton from 'src/components/BackButton';

const FallbackTextWrapper = styled.div`
  display: flex;
  justify-content: center;
  align-self: center;
  margin-top: 5rem;
  margin-bottom: 1rem;
  color: #a6a3a3;
`;

const DeploymentDetail = () => {
  const {t} = useTranslation();
  const {exerciseId, deploymentId}
  = useParams<DeploymentDetailRouteParameters>();
  const {data: tloMap} = useGetTLOsQuery(
    definedOrSkipToken(exerciseId, deploymentId));

  if (exerciseId && deploymentId) {
    return (
      <PageHolder>
        <BackButton/>
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
    <FallbackTextWrapper>
      {t('exercises.noDeploymentInfo')}
    </FallbackTextWrapper>
  );
};

export default DeploymentDetail;
