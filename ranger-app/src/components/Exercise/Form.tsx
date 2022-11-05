import React from 'react';
import type {SubmitHandler} from 'react-hook-form';
import {useForm, Controller} from 'react-hook-form';
import {Button, FormGroup, InputGroup, Intent} from '@blueprintjs/core';
import {AppToaster} from 'src/components/Toaster';
import type {Exercise, UpdateExercise} from 'src/models/exercise';
import {useUpdateExerciseMutation} from 'src/slices/apiSlice';
import Editor from '@monaco-editor/react';
import styled from 'styled-components';
import {useTranslation} from 'react-i18next';

const EditorHolder = styled.div`
  height: 40vh;
`;

const ExerciseForm = ({exercise}: {exercise: Exercise}) => {
  const {t} = useTranslation();
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
              label={t('exercises.name')}
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
        render={({
          field: {onChange, value}, fieldState: {error},
        }) => {
          const intent = error ? Intent.DANGER : Intent.NONE;
          return (
            <FormGroup
              labelFor='sdl-schema'
              helperText={error?.message}
              intent={intent}
              label={t('exercises.scenarioSDL')}
            >
              <EditorHolder>
                <Editor
                  value={value}
                  defaultLanguage='yaml'
                  onChange={onChange}
                />
              </EditorHolder>
            </FormGroup>
          );
        }}
      />
      <Button
        large
        type='submit'
        intent='primary'
      >{t('common.submit')}
      </Button>
    </form>

  );
};

export default ExerciseForm;
