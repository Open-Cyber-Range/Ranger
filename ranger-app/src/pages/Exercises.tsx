import React from 'react';
import {Intent} from '@blueprintjs/core';
import List from 'src/components/Exercise/List';
import PageHolder from 'src/components/PageHolder';
import {AppToaster} from 'src/components/Toaster';
import type {NewExercise} from 'src/models/Exercise';
import {useAddExerciseMutation} from 'src/slices/apiSlice';
import Header from 'src/components/Header';
import {useTranslation} from 'react-i18next';

const Exercise = () => {
  const {t} = useTranslation();
  const [addExercise, _newExercise] = useAddExerciseMutation();
  const addNewExercise = async (name: string) => {
    try {
      const exercise: NewExercise = {
        name,
      };
      await addExercise(exercise);

      AppToaster.show({
        icon: 'tick',
        intent: Intent.SUCCESS,
        message: `Exercise "${exercise.name}" added`,
      });
    } catch {
      AppToaster.show({
        icon: 'warning-sign',
        intent: Intent.DANGER,
        message: 'Failed to add the exercise',
      });
    }
  };

  return (
    <PageHolder>
      <Header
        headerTitle={t('exercises.title')}
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
