import {Button, Card} from '@blueprintjs/core';
import type {ReactElement, ReactNode} from 'react';
import {useState, useEffect} from 'react';
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
  const [isOpen, setIsOpen] = useState(true);
  const [style, setStyle] = useState('shrunk');
  const navigate = useNavigate();

  const deleteDeployment = () => {};

  const routeChange = () => {
    const path = exercise.name;
    navigate(path);
  };

  return (
    <div>
      <Card elevation={2} >
        <div className='float-right' >
          <div className='divider' />
          <Button onClick={deleteDeployment} intent='danger'> Delete</Button>
        </div>
        {exercise.name}  <br/>
        <div hidden={isOpen} >Content: {exercise.content} </div>

      </Card>
    </div>

  );
}

/*
Pub struct Deployment {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub name: String,
    pub sdl_schema: String,
    pub deployment_group: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
*/
/*
pub struct NewDeployment {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub name: String,
    pub sdl_schema: String,
    pub deployment_group: Option<String>,
}
*/
function ListDeployments() {
  const [payload, setPayload] = useState<ExerciseCard[]>([]);

  useEffect(() => {
    const fetchData = async () => (axios('../Mock_Exercises.json'));
    fetchData().then(response => {
      setPayload(response.data);
    }).catch(error => {
      throw new Error('Error retrieving exercise data');
    });
  }, []);

  return (
    <List items={payload} render={exercise => cardRender(exercise) }/>

  );
}

export default ListDeployments;
