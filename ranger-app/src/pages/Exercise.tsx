import type {SubmitHandler} from 'react-hook-form';
import {useForm, Controller} from 'react-hook-form';
import axios from 'axios';
import {Button, FormGroup, InputGroup, Intent, TextArea} from '@blueprintjs/core';
import styled from 'styled-components';
import {AppToaster} from '../components/Toaster';

const ExerciseWrapper = styled.div`
  padding: 2rem;
  max-width: 50rem;
`;

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
    <ExerciseWrapper >
      <h3>Add new exercise</h3>
      <form className='ExerciseForm' onSubmit={handleSubmit(onSubmit)} >
        <Controller
          control={control}
          name='name'
          rules={{required: 'Exercise must have a name'}}
          render={({field: {onChange, onBlur, ref, value}, fieldState: {error}}) => {
            const intent = error ? Intent.DANGER : Intent.NONE;
            return (
              <FormGroup labelFor='execise-name' labelInfo='(required)' helperText={error?.message} intent={intent} label='Exercise name'>
                <InputGroup
                  intent={intent}
                  value={value}
                  onChange={onChange}
                  onBlur={onBlur}
                  inputRef={ref}
                  id='execise-name'
                  large
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
                  intent={intent}
                  value={value}
                  onChange={onChange}
                  onBlur={onBlur}
                  inputRef={ref}
                  id='scenario'
                  small
                  fill
                  growVertically
                  rows={20}
                />
              </FormGroup>
            );
          }}
        />
        <Button type='submit' intent='primary'> Submit </Button>
      </form>
    </ExerciseWrapper>
  );
};

export default ExerciseForm;
