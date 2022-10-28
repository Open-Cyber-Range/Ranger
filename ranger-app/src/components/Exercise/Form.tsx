import React from 'react';
import type {SubmitHandler} from 'react-hook-form';
import {useForm, Controller} from 'react-hook-form';
import {
  Button,
  FormGroup,
  InputGroup,
  Intent,
  TextArea,
} from '@blueprintjs/core';
import {AppToaster} from 'src/components/Toaster';
import type {Exercise, UpdateExercise} from 'src/models/Exercise';
import {useUpdateExerciseMutation} from 'src/slices/apiSlice';

const ExerciseForm = ({exercise}: {exercise: Exercise}) => {
  const {handleSubmit, control} = useForm<UpdateExercise>({
    defaultValues: {
      name: exercise.name,
      sdlSchema: exercise.sdlSchema ?? '',
    },
  });
  const [updateExercise, _newExercise] = useUpdateExerciseMutation();

  const onSubmit: SubmitHandler<UpdateExercise> = async exerciseUpdate => {
    try {
      await updateExercise({exerciseUpdate, exerciseId: exercise.id});
      AppToaster.show({
        icon: 'tick',
        intent: Intent.SUCCESS,
        message: `Exercise "${exerciseUpdate.name}" updated`,
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
    <form className='ExerciseForm' onSubmit={handleSubmit(onSubmit)}>
      <Controller
        control={control}
        name='name'
        rules={{required: 'Exercise must have a name'}}
        render={({
          field: {onChange, onBlur, ref, value}, fieldState: {error},
        }) => {
          const intent = error ? Intent.DANGER : Intent.NONE;
          return (
            <FormGroup
              labelFor='execise-name'
              labelInfo='(required)'
              helperText={error?.message}
              intent={intent}
              label='Exercise name'
            >
              <InputGroup
                large
                intent={intent}
                value={value}
                inputRef={ref}
                id='execise-name'
                onChange={onChange}
                onBlur={onBlur}
              />
            </FormGroup>
          );
        }}
      />
      <Controller
        control={control}
        name='sdlSchema'
        rules={{required: 'Exercise must have a SDL schema'}}
        render={({
          field: {onChange, onBlur, ref, value}, fieldState: {error},
        }) => {
          const intent = error ? Intent.DANGER : Intent.NONE;
          return (
            <FormGroup
              labelFor='sdl-schema'
              labelInfo='(required)'
              helperText={error?.message}
              intent={intent}
              label='Scenario SDL'
            >
              <TextArea
                small
                fill
                growVertically
                intent={intent}
                value={value}
                inputRef={ref}
                id='sdl-schema'
                rows={20}
                onChange={onChange}
                onBlur={onBlur}
              />
            </FormGroup>
          );
        }}
      />
      <Button
        large
        type='submit'
        intent='primary'
      > Submit
      </Button>
    </form>

  );
};

export default ExerciseForm;
