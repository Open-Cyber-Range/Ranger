import React from 'react';
import List from 'src/components/Exercise/List';
import PageHolder from 'src/components/PageHolder';
import {toastSuccess, toastWarning} from 'src/components/Toaster';
import type {NewExercise} from 'src/models/exercise';
import {useAddExerciseMutation} from 'src/slices/apiSlice';
import Header from 'src/components/Header';

const Exercise = () => {
  const [addExercise, _newExercise] = useAddExerciseMutation();
  const addNewExercise = async (name: string) => {
    try {
      const newExercise: NewExercise = {
        name,
      };
      const exercise = await addExercise(newExercise).unwrap();
      if (exercise) {
        toastSuccess(`Exercise "${exercise.name}" added`);
      }
    } catch {
      toastWarning('Failed to add the exercise');
    }
  };

  return (
    <PageHolder>
      <Header
        headerTitle='Exercises'
        dialogTitle='Add Exercise'
        buttonTitle='Add Exercise'
        onSubmit={async name => {
          await addNewExercise(name);
        }}/>
      <List/>
    </PageHolder>
  );
};

export default Exercise;
