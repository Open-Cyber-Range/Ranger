import {skipToken} from '@reduxjs/toolkit/dist/query';
import React from 'react';
import {useParams} from 'react-router-dom';
import SendEmail from 'src/components/Email/SendEmail';
import PageHolder from 'src/components/PageHolder';
import {type ExerciseDetailRouteParameters} from 'src/models/routes';
import {useGetExerciseQuery} from 'src/slices/apiSlice';

const Email = () => {
  const {exerciseId} = useParams<ExerciseDetailRouteParameters>();
  const {data: exercise} = useGetExerciseQuery(exerciseId ?? skipToken);

  if (exercise) {
    return (
      <PageHolder>
        <SendEmail exercise={exercise}/>
      </PageHolder>
    );
  }

  return null;
};

export default Email;
