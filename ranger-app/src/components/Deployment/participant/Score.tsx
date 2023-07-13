import React from 'react';
import {H1} from '@blueprintjs/core';
import ParticipantDeploymentGraph from 'src/components/Scoring/participant/Graph';

const ParticipantScore = ({
  exerciseId, deploymentId}: {exerciseId: string; deploymentId: string;
}) => (
  <div>
    <H1>Score</H1>
    <ParticipantDeploymentGraph
      exerciseId={exerciseId}
      deploymentId={deploymentId}
    />
  </div>
);

export default ParticipantScore;
