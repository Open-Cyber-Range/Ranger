import React from 'react';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {useTranslation} from 'react-i18next';
import {
  useAdminGetDeploymentScenarioQuery,
  useAdminGetDeploymentScoresQuery,
} from 'src/slices/apiSlice';
import {findLatestScoresByVms, groupBy, roundToDecimalPlaces} from 'src/utils';
import {type TrainingLearningObjective} from 'src/models/scenario';
import {H5} from '@blueprintjs/core';
import {sortByProperty} from 'sort-by-property';

const TloTableRow = ({exerciseId, deploymentId, tloKey, tlo}:
{exerciseId: string;
  deploymentId: string;
  tloKey: string;
  tlo: TrainingLearningObjective | undefined;
}) => {
  const {t} = useTranslation();
  const queryArguments = exerciseId && deploymentId
    ? {exerciseId, deploymentId} : skipToken;
  const {data: scenario} = useAdminGetDeploymentScenarioQuery(queryArguments);
  const {data: scores} = useAdminGetDeploymentScoresQuery(queryArguments);
  const scenarioEvaluations = scenario?.evaluations;
  const scenarioMetrics = scenario?.metrics;

  if (tlo && scenarioEvaluations && scenarioMetrics) {
    const tloEvaluation = scenarioEvaluations[tlo.evaluation];
    if (tloEvaluation && scores) {
      const scoresByMetric = groupBy(scores, score => score.metricName);

      return (
        <tr key={tloKey}>
          <td>
            <H5>{tlo.name ?? tloKey}</H5>
            <p>{tlo.description}</p>
          </td>
          <td>
            <H5>{tloEvaluation.name ?? tlo.evaluation}</H5>
            <p>{tloEvaluation.description}</p>
          </td>
          <td className='flex flex-col items-stretch'>
            <table>
              <tbody>
                {tloEvaluation.metrics.map(metricKey => {
                  const metric = scenarioMetrics[metricKey];
                  const metricReference = metric.name ?? metricKey;
                  const scores = scoresByMetric[metricReference];

                  if (scores && scores.length > 0) {
                    const latestScoresByVm = findLatestScoresByVms(scores);
                    latestScoresByVm.sort(sortByProperty('vmName', 'desc'));

                    return (
                      <tr key={metricKey} className='text-left'>
                        {latestScoresByVm.map(element => (
                          <td key={element.id} className='pl-4'>
                            {metricReference} - {element.vmName}:
                            {roundToDecimalPlaces(
                              element.value)} {t('tloTable.points')}
                          </td>
                        ))}
                      </tr>
                    );
                  }

                  return (
                    <tr key={metricKey}>
                      <td key={metricKey} className='text-left pl-5'>
                        {metricReference} - {t('tloTable.noMetricData')}
                      </td>
                    </tr>
                  );
                },
                )}
              </tbody>
            </table>
          </td>
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
