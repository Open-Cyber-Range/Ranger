import React from 'react';
import styled from 'styled-components';
import '@blueprintjs/popover2/lib/css/blueprint-popover2.css';
import {
  useGetDeploymentEntitiesQuery,
  useGetDeploymentGoalsQuery,
} from 'src/slices/apiSlice';
import {exerciseRoleOrder} from 'src/models/entity';
import ScoreTagBody from './ScoreTagBody';

const TagGroupWrapper = styled.div`
  display: flex;
  margin: 0.25rem;
  margin-top: auto;
  margin-bottom: auto;
`;

const TagWrapper = styled.div`
  display: flex;
  margin-right: 0.25rem; 
`;

const ScoreTags = ({exerciseId, deploymentId}:
{exerciseId: string;
  deploymentId: string;
}) => {
  const {data: entities}
  = useGetDeploymentEntitiesQuery({exerciseId, deploymentId});

  const {data: goals}
  = useGetDeploymentGoalsQuery({exerciseId, deploymentId});

  if (entities && goals) {
    const roles = Object.values(entities)
      .filter(entity => entity.role)
      .map(entity => entity.role!);
    roles.sort((a, b) => exerciseRoleOrder[a] - exerciseRoleOrder[b]);

    return (
      <TagGroupWrapper>
        {roles.map(role => (
          <TagWrapper key={role}>
            <ScoreTagBody
              key={role}
              exerciseId={exerciseId}
              deploymentId={deploymentId}
              role={role}
            />
          </TagWrapper>
        ))}
      </TagGroupWrapper>
    );
  }

  return (
    null
  );
};

export default ScoreTags;
