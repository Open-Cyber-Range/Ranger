import React from 'react';
import type {TrainingLearningObjective} from 'src/models/tlo';
import EvaluationTd from './EvaluationTd';

const TloRow = ({exerciseId, deploymentId, tloName, tloMap}:
{exerciseId: string;
  deploymentId: string;
  tloName: string;
  tloMap: Record<string, TrainingLearningObjective>;
}) => {
  if (tloMap[tloName]) {
    return (
      <tr key={tloName}>
        <td>
          <div>{tloName}</div>
          <div>{tloMap[tloName].description}</div>
        </td>
        <EvaluationTd
          key={tloName}
          exerciseId={exerciseId}
          deploymentId={deploymentId}
          tloName={tloName}
          tloMap={tloMap}
        />
      </tr>
    );
  }

  return (
    <tr>
      Nothing
    </tr>
  );
};

export default TloRow;
