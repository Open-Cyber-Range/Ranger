import React from 'react';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {useTranslation} from 'react-i18next';
import {type TrainingLearningObjective} from 'src/models/scenario/tlo';
import {
  useGetDeploymentScoresQuery,
  useGetEvaluationQuery,
} from 'src/slices/apiSlice';
import {groupBy} from 'src/utils';
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
  const {data: scores} = useGetDeploymentScoresQuery(
    exerciseId && deploymentId ? {exerciseId, deploymentId} : skipToken);

  if (evaluation && scores) {
    const scoresByMetric = groupBy(scores, score => score.metricName);

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
              metricName={metricName}
              scores={scoresByMetric[metricName]}
            />
          ))}
        </td>

      </>
    );
  }

  return (
    <td>
      {t('tloTable.noEvaluations')}
    </td>
  );
};

export default TloDetails;
