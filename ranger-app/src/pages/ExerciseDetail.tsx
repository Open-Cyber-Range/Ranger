import React from 'react';
import {useParams} from 'react-router-dom';
import ListDeployments from 'src/components/DeploymentCard';
import ExerciseForm from 'src/components/Exercise/Form';
import type {ExerciseDetailRouteParameters} from 'src/models/Routes';
import PageHolder from 'src/components/PageHolder';
import {useGetDeploymentsQuery} from 'src/slices/apiSlice';
import {skipToken} from '@reduxjs/toolkit/dist/query';

const ExerciseDetail = () => {
  const {exerciseId} = useParams<ExerciseDetailRouteParameters>();
  const {data: deployments} = useGetDeploymentsQuery(exerciseId ?? skipToken);
  return (
    <PageHolder>
      Exercise ID:  &quot;{exerciseId}&quot;
      {JSON.stringify(deployments)}

      <ExerciseForm/>
      <br/>
      <ListDeployments/>
    </PageHolder>
  );
};

export default ExerciseDetail;
