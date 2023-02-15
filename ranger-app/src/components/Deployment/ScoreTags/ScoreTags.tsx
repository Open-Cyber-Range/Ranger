
import React from 'react';
import styled from 'styled-components';
import '@blueprintjs/popover2/lib/css/blueprint-popover2.css';
import {
  useGetDeploymentEntitiesQuery,
  useGetDeploymentGoalsQuery,
} from 'src/slices/apiSlice';
import ScoreTagBuilder from './ScoreTagBuilder';

const TagWrapper = styled.div`
  display: flex;
  margin: 2px;
  margin-top: auto;
  margin-bottom: auto;
`;

const ScoreTags = ({exerciseId, deploymentId}:
{exerciseId: string;
  deploymentId: string;
}) => {
  const {data: entities}
  = useGetDeploymentEntitiesQuery({exerciseId, deploymentId});

  const {data: goals}
  = useGetDeploymentGoalsQuery({exerciseId, deploymentId});

  if (!entities || !goals) {
    return (
      null
    );
  }

  return (
    <TagWrapper>
      <ScoreTagBuilder
        exerciseId={exerciseId}
        deploymentId={deploymentId}
        entities={entities}
      />
    </TagWrapper>
  );
};

export default ScoreTags;
