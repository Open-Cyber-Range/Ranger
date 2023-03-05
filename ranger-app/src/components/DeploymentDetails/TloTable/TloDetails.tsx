import React from 'react';
import {useTranslation} from 'react-i18next';
import type {TrainingLearningObjective} from 'src/models/tlo';
import {useGetEvaluationQuery} from 'src/slices/apiSlice';
import MetricInfo from './MetricInfo';

const TloDetails = ({exerciseId, deploymentId, tloName, tloMap}:
{exerciseId: string;
  deploymentId: string;
  tloName: string;
  tloMap: Record<string, TrainingLearningObjective>;
}) => {
  const {t} = useTranslation();
  const {data: evaluation} = useGetEvaluationQuery(
    {exerciseId,
      deploymentId,
      tloName});

  if (evaluation) {
    return (
      <>
        <td key={tloMap[tloName].evaluation}>
          <div>{tloMap[tloName].evaluation}</div>
          <div>{evaluation.description}</div>
        </td>
        <td key={tloName}>
          {evaluation.metrics.map(metricName => (
            <MetricInfo
              key={metricName}
              exerciseId={exerciseId}
              deploymentId={deploymentId}
              tloName={tloName}
              metricName={metricName}/>
          ))}
        </td>

      </>
    );
  }

  return (
    <td>
      {t('error.none')}
    </td>
  );
};

export default TloDetails;
