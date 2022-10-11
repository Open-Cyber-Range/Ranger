import type {SubmitHandler} from 'react-hook-form';
import {useForm} from 'react-hook-form';
import axios from 'axios';
import {Button, FormGroup, InputGroup, Intent} from '@blueprintjs/core';
import {AppToaster} from '../components/Toaster';

type Exercise = {
  name: string;
  scenario: string;
};

const ExerciseForm = () => {
  const {register, handleSubmit, formState: {errors}} = useForm<Exercise>();

  const onSubmit: SubmitHandler<Exercise> = async exercise => {
    try {
      const response = axios.post<Exercise>('api/v1/exercise');
      AppToaster.show({
        icon: 'tick',
        intent: Intent.SUCCESS,
        message: 'Exercise added!',
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
    <div >
      <h3>Add new exercise</h3>
      <form className='ExerciseForm' onSubmit={handleSubmit(onSubmit)} >
        <FormGroup
          label='Exercise name'
          labelFor='exercise-name'
          inline>
          <InputGroup
            id='exercise-name'
            placeholder='exercise-1'
          />
        </FormGroup>
        <FormGroup
          label='Scenario yaml'
          labelFor='scenario'
          inline>
          <InputGroup
            id='scenario'
            placeholder='scenario: ...'
          />
        </FormGroup>
        <Button type='submit' > Submit </Button>
      </form>

    </div>
  );
};

export default ExerciseForm;
