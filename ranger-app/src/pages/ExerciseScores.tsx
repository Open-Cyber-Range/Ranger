import React from 'react';
import {useParams} from 'react-router-dom';
import type {ExerciseDetailRouteParameters} from 'src/models/routes';
import PageHolder from 'src/components/PageHolder';
import {useTranslation} from 'react-i18next';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {useGetDeploymentsQuery, useGetExerciseQuery} from 'src/slices/apiSlice';
import styled from 'styled-components';
import DeploymentScoresTable from
  'src/components/ExerciseScores/ScoresTable/DeploymentScoresTable';

const Wrapper = styled.div`
  display: flex;
  flex-direction: column;
  text-align: center;
`;

const FallbackTextWrapper = styled.div`
  display: flex;
  justify-content: center;
  align-self: center;
  margin-top: 5rem;
  margin-bottom: 1rem;
  color: #a6a3a3;
`;

const ExerciseScores = () => {
  const {t} = useTranslation();
  const {exerciseId} = useParams<ExerciseDetailRouteParameters>();
  const {data: deployments} = useGetDeploymentsQuery(exerciseId ?? skipToken);
  const {data: exercise} = useGetExerciseQuery(exerciseId ?? skipToken);

  if (!deployments || !exerciseId) {
    return (
      <FallbackTextWrapper>
        {t('exercises.noDeployments')}
      </FallbackTextWrapper>
    );
  }

  return (
    <PageHolder>
      <Wrapper>
        <h1>{exercise?.name ?? ''}</h1>
        <DeploymentScoresTable deployments={deployments}/>
      </Wrapper>
    </PageHolder>
  );
};

export default ExerciseScores;
