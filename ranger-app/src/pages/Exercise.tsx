import React from 'react';
import type {SubmitHandler} from 'react-hook-form';
import {useForm, Controller} from 'react-hook-form';
import axios from 'axios';
import {Button, FormGroup, H2, InputGroup, Intent, TextArea} from '@blueprintjs/core';
import {AppToaster} from '../components/Toaster';
import ExerciseList from '../components/Exercise/List';
import PageHolder from '../components/PageHolder';

type Exercise = {
  name: string;
  scenario: string;
};

const ExerciseForm = () => {
  const {handleSubmit, control} = useForm<Exercise>({
    defaultValues: {
      name: '',
      scenario: '',
    },
  });

  const onSubmit: SubmitHandler<Exercise> = async exercise => {
    try {
      await axios.post<Exercise>('api/v1/exercise', exercise);
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
      <H2>Add new exercise</H2>
      <form className='ExerciseForm' onSubmit={handleSubmit(onSubmit)}>
        <Controller
          control={control}
          name='name'
          rules={{required: 'Exercise must have a name'}}
          render={({field: {onChange, onBlur, ref, value}, fieldState: {error}}) => {
            const intent = error ? Intent.DANGER : Intent.NONE;
            return (
              <FormGroup labelFor='execise-name' labelInfo='(required)' helperText={error?.message} intent={intent} label='Exercise name'>
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
          name='scenario'
          rules={{required: 'Exercise must have a scenario'}}
          render={({field: {onChange, onBlur, ref, value}, fieldState: {error}}) => {
            const intent = error ? Intent.DANGER : Intent.NONE;
            return (
              <FormGroup labelFor='scenario' labelInfo='(required)' helperText={error?.message} intent={intent} label='Scenario SDL'>
                <TextArea
                  small
                  fill
                  growVertically
                  intent={intent}
                  value={value}
                  inputRef={ref}
                  id='scenario'
                  rows={20}
                  onChange={onChange}
                  onBlur={onBlur}
                />
              </FormGroup>
            );
          }}
        />
        <Button type='submit' intent='primary'> Submit </Button>
        <ExerciseList/>
      </form>
    </PageHolder>
  );
};

export default ExerciseForm;
