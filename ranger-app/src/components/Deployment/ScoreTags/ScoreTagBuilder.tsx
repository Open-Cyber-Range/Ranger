
import React from 'react';
import styled from 'styled-components';
import '@blueprintjs/popover2/lib/css/blueprint-popover2.css';
import type {Entities} from 'src/models/entity';
import {ExerciseRole} from 'src/models/entity';
import ScoreTagData from './ScoreTagData';

const TagWrapper = styled.div`
  display: flex;
  margin-right: 0.25rem; 
`;

const ScoreTagBuilder = ({exerciseId, deploymentId, entities}:
{exerciseId: string;
  deploymentId: string;
  entities: Entities;
}) => {
  const roles: ExerciseRole[] = [];

  for (const entityName in entities) {
    if (Object.prototype.hasOwnProperty.call(entities, entityName)) {
      const role = entities[entityName].role;

      if (role) {
        roles.push(role);
      }
    }
  }

  const rolesOrder = Object.values(ExerciseRole);
  const sortedroles = roles
    .sort((a, b) => rolesOrder.indexOf(a) - rolesOrder.indexOf(b));

  return (
    <>
      {sortedroles.map(role => (
        <TagWrapper key={role}>
          <ScoreTagData
            key={role}
            exerciseId={exerciseId}
            deploymentId={deploymentId}
            role={role}
          />
        </TagWrapper>
      ))}
    </>
  );
};

export default ScoreTagBuilder;
