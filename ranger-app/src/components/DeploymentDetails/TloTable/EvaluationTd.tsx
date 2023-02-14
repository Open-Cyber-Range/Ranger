import React from 'react';
import type {TrainingLearningObjective} from 'src/models/tlo';
import {useGetEvaluationQuery} from 'src/slices/apiSlice';
import MetricsTd from './MetricsTd';

const EvaluationTd = ({exerciseId, deploymentId, tloName, tloMap}:
{exerciseId: string;
  deploymentId: string;
  tloName: string;
  tloMap: Record<string, TrainingLearningObjective>;
}) => {
  const {data: evaluation} = useGetEvaluationQuery(
    {exerciseId,
      deploymentId,
      tloName});

  if (evaluation) {
    return (
      <>
        <td key={tloName}>
          <div>{tloMap[tloName].evaluation}</div>
          <div>{evaluation.description}</div>
        </td>
        <MetricsTd
          exerciseId={exerciseId}
          deploymentId={deploymentId}
          tloName={tloName}
          metrics={evaluation.metrics}/>
      </>
    );
  }

  return (
    <td>
      No evaluation to display
    </td>
  );
};

export default EvaluationTd;
