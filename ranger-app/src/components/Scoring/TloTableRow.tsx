import React from 'react';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {useTranslation} from 'react-i18next';
import {
  useGetDeploymentScenarioQuery,
  useGetDeploymentScoresQuery,
} from 'src/slices/apiSlice';
import {findLatestScoresByVms, groupBy, roundToDecimalPlaces} from 'src/utils';
import {type TrainingLearningObjective} from 'src/models/scenario';
import {H5} from '@blueprintjs/core';

const TloTableRow = ({exerciseId, deploymentId, tloKey, tlo}:
{exerciseId: string;
  deploymentId: string;
  tloKey: string;
  tlo: TrainingLearningObjective | undefined;
}) => {
  const {t} = useTranslation();
  const queryArguments = exerciseId && deploymentId
    ? {exerciseId, deploymentId} : skipToken;
  const {data: scenario} = useGetDeploymentScenarioQuery(queryArguments);
  const {data: scores} = useGetDeploymentScoresQuery(queryArguments);

  if (tlo && scenario?.evaluations) {
    const tloEvaluation = scenario?.evaluations[tlo.evaluation];
    if (tloEvaluation && scores) {
      const scoresByMetric = groupBy(scores, score => score.metricName);

      return (
        <tr key={tloKey}>
          <td>
            <H5>{tlo.name ?? tloKey}</H5>
            <p>{tlo.description}</p>
          </td>
          <td>
            <H5>{tlo.evaluation}</H5>
            <p>{tloEvaluation.description}</p>
          </td>
          {tloEvaluation.metrics.map(metricName => {
            const scores = scoresByMetric[metricName];
            if (scores && scores.length > 0) {
              const latestScoresByVm = findLatestScoresByVms(scores);
              latestScoresByVm.sort((a, b) => a.vmName.localeCompare(b.vmName));

              return (
                <td key={metricName}>
                  {latestScoresByVm.map(element => (
                    <div key={element.id} className='pl-4'>
                      <li key={element.id}>
                        {metricName} - {element.vmName}: {roundToDecimalPlaces(
                          element.value)} {t('tloTable.points')}
                      </li>
                    </div>
                  ))}
                </td>
              );
            }

            return (
              <td key={metricName}>
                <div className='pl-4'>
                  <li>
                    {metricName} - {t('tloTable.noMetricData')}
                  </li>
                </div>
              </td>
            );
          },
          )}
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

export default TloTableRow;
