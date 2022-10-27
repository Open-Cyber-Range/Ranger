import React from 'react';
import {useParams} from 'react-router-dom';
import ListDeployments from 'src/components/DeploymentCard';
import ExerciseForm from 'src/components/Exercise/Form';
import type {ExerciseDetailRouteParameters} from 'src/models/Routes';
import PageHolder from 'src/components/PageHolder';

const ExerciseDetail = () => {
  const {exerciseId} = useParams<ExerciseDetailRouteParameters>();

  return (
    <PageHolder>
      Exercise ID:  &quot;{exerciseId}&quot;

      <ExerciseForm/>
      <br/>
      <ListDeployments/>
    </PageHolder>
  );
};

export default ExerciseDetail;
