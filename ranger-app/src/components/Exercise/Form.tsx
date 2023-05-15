import React, {useEffect} from 'react';
import type {SubmitHandler} from 'react-hook-form';
import {useForm, Controller} from 'react-hook-form';
import {Button, FormGroup, InputGroup, Intent} from '@blueprintjs/core';
import {toastSuccess, toastWarning} from 'src/components/Toaster';
import type {Exercise, UpdateExercise} from 'src/models/exercise';
import {useAdminUpdateExerciseMutation} from 'src/slices/apiSlice';
import Editor from '@monaco-editor/react';
import {useTranslation} from 'react-i18next';
import init, {parse_and_verify_sdl} from 'wasm-sdl-parser';

const ExerciseForm = ({exercise}: {exercise: Exercise}) => {
  const {t} = useTranslation();
  const {handleSubmit, control} = useForm<UpdateExercise>({
    defaultValues: {
      name: exercise.name,
      sdlSchema: exercise.sdlSchema ?? '',
    },
  });
  const [updateExercise, {isSuccess, error}] = useAdminUpdateExerciseMutation();

  const onSubmit: SubmitHandler<UpdateExercise> = async exerciseUpdate => {
    if (exerciseUpdate.sdlSchema) {
      try {
        parse_and_verify_sdl(exerciseUpdate.sdlSchema);
      } catch (error: unknown) {
        if (typeof error === 'string') {
          toastWarning(error);
        } else {
          toastWarning(t('exercises.sdlParsingFail'));
        }

        return;
      }
    }

    await updateExercise({exerciseUpdate, exerciseId: exercise.id});
  };

  useEffect(() => {
    const initializeSdlParser = async () => {
      await init();
    };

    initializeSdlParser()
      .catch(() => {
        toastWarning(t('exercises.sdlParserInitFail'));
      });
  }, [t]);

  useEffect(() => {
    if (isSuccess) {
      toastSuccess(t('exercises.updateSuccess', {
        exerciseName: JSON.stringify(exercise.name),
      }));
    }
  }, [isSuccess, t, exercise.name]);

  useEffect(() => {
    if (error) {
      if ('data' in error) {
        toastWarning(t('exercises.updateFail', {
          errorMessage: JSON.stringify(error.data),
        }));
      } else {
        toastWarning(t('exercises.updateFail'));
      }
    }
  }, [error, t]);

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
              <div className='h-[40vh]'>
                <Editor
                  value={value}
                  defaultLanguage='yaml'
                  onChange={onChange}
                />
              </div>
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
