import React, {useState} from 'react';
import {Button, H2, Intent} from '@blueprintjs/core';
import List from 'src/components/Exercise/List';
import NameDialog from 'src/components/NameDialog';
import PageHolder from 'src/components/PageHolder';
import styled from 'styled-components';
import axios from 'axios';
import {AppToaster} from 'src/components/Toaster';
import type {NewExercise} from 'src/models/Exercise';

const Header = styled.div`
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  margin-bottom: 4rem;
`;

const Exercise = () => {
  const [isOpen, setIsOpen] = useState(false);
  const addNewExercise = async (name: string) => {
    try {
      const exercise: NewExercise = {
        name,
      };
      await axios.post<NewExercise>('api/v1/exercise', exercise);
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
      <Header>
        <H2>Exercises</H2>
        <Button
          large
          icon='add'
          intent='success'
          text='Add new exercise'
          onClick={() => {
            setIsOpen(true);
          }}/>
      </Header>
      <NameDialog
        title='Add exercise'
        isOpen={isOpen}
        onCancel={() => {
          setIsOpen(false);
        }}
        onSumbit={async value => {
          setIsOpen(false);
          await addNewExercise(value);
        }}/>
      <List/>
    </PageHolder>
  );
};

export default Exercise;
