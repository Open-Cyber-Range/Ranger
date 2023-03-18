import React from 'react';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {useTranslation} from 'react-i18next';
import {type TrainingLearningObjective} from 'src/models/scenario/tlo';
import {
  useGetDeploymentSchemaQuery,
  useGetDeploymentScoresQuery,
} from 'src/slices/apiSlice';
import {findLatestScoresByVms, groupBy, roundToDecimalPlaces} from 'src/utils';
import {type Score} from 'src/models/score';

const TloRow = ({exerciseId, deploymentId, tloKey, tlo}:
{exerciseId: string;
  deploymentId: string;
  tloKey: string;
  tlo: TrainingLearningObjective;
}) => {
  const {t} = useTranslation();
  const queryArguments = exerciseId && deploymentId
    ? {exerciseId, deploymentId} : skipToken;
  const {data: schema} = useGetDeploymentSchemaQuery(queryArguments);
  const {data: scores} = useGetDeploymentScoresQuery(queryArguments);

  const MetricsCell = ({scores, metricName}:
  {scores: Score[]; metricName: string}) => {
    if (scores.length > 0) {
      const latestScoresByVm = findLatestScoresByVms(scores);
      latestScoresByVm.sort((a, b) => a.vmName.localeCompare(b.vmName));

      return (
        <td>
          {latestScoresByVm.map(element => (
            <li key={element.id}>
              {metricName} - {element.vmName}: {roundToDecimalPlaces(
                element.value)} {t('tloTable.points')}
            </li>
          ))}
        </td>
      );
    }

    return (
      <td>
        <li>
          {t('tloTable.noMetricData')}
        </li>
      </td>
    );
  };

  if (tlo) {
    const tloEvaluation = schema?.evaluations[tlo.evaluation];
    if (tloEvaluation && scores) {
      const scoresByMetric = groupBy(scores, score => score.metricName);

      return (
        <tr key={tloKey}>
          <td>
            <h4>{tlo.name ?? tloKey}</h4>
            <p>{tlo.description}</p>
          </td>
          <td>
            <h4>{tlo.evaluation}</h4>
            <p>{tloEvaluation.description}</p>
          </td>
          {tloEvaluation.metrics.map(metricName => (
            <MetricsCell
              key={metricName}
              metricName={metricName}
              scores={scoresByMetric[metricName]}
            />
          ))}
        </tr>
      );
    }

    return (
      <tr>
        <td>
          <h3>{tlo.name ?? tloKey}</h3>
          <p>{tlo.description}</p>
        </td>
        <td>
          {t('tloTable.noEvaluations')}
        </td>
        <td/>
      </tr>
    );
  }

  return (
    <tr>
      <td>
        {t('tloTable.noTlos')}
      </td>
      <td/>
      <td/>
    </tr>
  );
};

export default TloRow;
