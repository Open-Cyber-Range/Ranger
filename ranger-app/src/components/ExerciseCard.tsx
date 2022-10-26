
import {Button, Card} from '@blueprintjs/core';
import type {ReactElement, ReactNode} from 'react';
import {useState, useEffect} from 'react';
import './ExerciseCard.css';
import axios from 'axios';
import {useNavigate} from 'react-router-dom';

export type ExerciseCard = {
  id: number;
  open?: boolean;
  name: string;
  content: string;
};

export type Deployment = {
  id: number;
  exerciseName: string;
  name: string;
};

export const List: <T>({
  items,
  render,
}: {
  items: T[];
  render: (item: T) => ReactNode;

}) => ReactElement = ({items, render}) => (
  <>
    {items.map((item, index) => (
      <div key={index}>{render(item)}</div>
    ))}
  </>
);

export function cardRender(exercise: ExerciseCard) {
  const navigate = useNavigate();

  const routeChange = () => {
    const path = exercise.name;
    navigate(path);
  };

  return (
    <div className='wrapper'>
      <Card interactive={true} elevation={2} >
        <div className='float-right' >
          <Button intent='primary' onClick={routeChange}>
            View
          </Button>
          <div className='divider' />
          <Button intent='danger'> Delete</Button>
        </div>
        {exercise.name}  <br/>
      </Card>
    </div>

  );
}

function ListExercises() {
  const [payload, setPayload] = useState<ExerciseCard[]>([]);

  useEffect(() => {
    const fetchData = async () => (axios.get('api/v1/exercise'));
    fetchData().then(response => {
      setPayload(response.data);
    }).catch(error => {
      throw new Error('Error retrieving exercise data');
    });
  }, []);

  return (
    <List items={payload} render={exercise => CardRender(exercise) }/> // eslint-disable-line new-cap

  );
}

export default ListExercises;
