import {Button, FormGroup, InputGroup, Intent} from '@blueprintjs/core';
import axios from 'axios';
import React from 'react';
import type {SubmitHandler} from 'react-hook-form';
import {Controller, useForm} from 'react-hook-form';
import {useParams} from 'react-router-dom';
import styled from 'styled-components';
import ListDeployments from 'src/components/DeploymentCard';
import {AppToaster} from 'src/components/Toaster';

const ExerciseWrapper = styled.div`
  padding: 2rem;
  max-width: 50rem;
`;

type Deployment = {
  name: string;
};

const onSubmit: SubmitHandler<Deployment> = async deployment => {
  try {
    await axios.post<Deployment>('api/v1/exercise', deployment);
    AppToaster.show({
      icon: 'tick',
      intent: Intent.SUCCESS,
      message: `Deployment "${deployment.name}" added`,
    });
  } catch {
    AppToaster.show({
      icon: 'warning-sign',
      intent: Intent.DANGER,
      message: 'Failed to add the exercise',
    });
  }
};

type RouteParameters = {
  exerciseName: string;
};

const DeploymentForm = () => {
  const parameters = useParams<RouteParameters>();
  const {handleSubmit, control} = useForm<Deployment>({
    defaultValues: {
      name: '',
    },
  });

  return (
    <ExerciseWrapper>
      herro  &quot;{parameters.exerciseName}&quot;
      <div>
        <form onSubmit={handleSubmit(onSubmit)}>
          <Controller
            control={control}
            name='name'
            rules={{required: 'Deployment must have a name'}}
            render={({
              field: {onChange, onBlur, ref, value}, fieldState: {error},
            }) => {
              const intent = error ? Intent.DANGER : Intent.NONE;
              return (
                <FormGroup
                  labelFor='deployment-name'
                  labelInfo='(required)'
                  helperText={error?.message}
                  intent={intent}
                  label='Deployment name'
                >
                  <InputGroup
                    large
                    intent={intent}
                    value={value}
                    inputRef={ref}
                    id='deployment-name'
                    onChange={onChange}
                    onBlur={onBlur}
                  />
                </FormGroup>
              );
            }}
          />

          <Button type='submit' intent='primary'> Add </Button>
        </form>
        <br/>
        <ListDeployments/>
      </div>
    </ExerciseWrapper>
  );
};

export default DeploymentForm;
