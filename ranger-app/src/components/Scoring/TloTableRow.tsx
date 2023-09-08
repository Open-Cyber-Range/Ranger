import React from 'react';
import {useTranslation} from 'react-i18next';
import {findLatestScoresByVms, groupBy, roundToDecimalPlaces} from 'src/utils';
import {
  type ScoringMetadata,
  type TrainingLearningObjective,
} from 'src/models/scenario';
import {H5} from '@blueprintjs/core';
import {sortByProperty} from 'sort-by-property';
import {type Score} from 'src/models/score';

const TloTableRow = ({scoringData, scores, tloKey, tlo}:
{scoringData: ScoringMetadata;
  scores: Score[] | undefined;
  tloKey: string;
  tlo: TrainingLearningObjective | undefined;
}) => {
  const {t} = useTranslation();

  if (tlo && scores) {
    const tloEvaluation = scoringData.evaluations[tlo.evaluation];
    const scoresByMetric = groupBy(scores, score => score.metricName);

    return (
      <tr key={tloKey} className='overflow-y-auto even:bg-slate-200'>
        <td className='w-1/3 px-6 py-4 border-r border-neutral-500'>
          <H5>{tlo.name ?? tloKey}</H5>
          <p className='max-h-32 overflow-auto break-words'>
            {tlo.description}
          </p>
        </td>
        <td className='w-1/3 px-6 py-4 overflow-y-auto border-r border-neutral-500'>
          <H5>{tloEvaluation.name ?? tlo.evaluation}</H5>
          <p className='max-h-32 overflow-auto break-words'>
            {tloEvaluation.description}
          </p>
        </td>
        <td className='w-1/3 py-1' colSpan={3}>
          <table className='w-full'>
            <tbody>
              <tr className='flex flex-col'>
                {tloEvaluation.metrics.map(metricKey => {
                  const metric = scoringData.metrics[metricKey];
                  const metricReference = metric.name ?? metricKey;
                  const scores = scoresByMetric[metricReference];

                  if (scores && scores.length > 0) {
                    const latestScoresByVm = findLatestScoresByVms(scores);
                    latestScoresByVm.sort(sortByProperty('vmName', 'desc'));

                    return (
                      <td key={metricKey}>
                        {latestScoresByVm.map(element => (
                          <table key={element.id} className='w-full'>
                            <tbody>
                              <tr>
                                <td
                                  key={element.id}
                                  className='pl-1 py-1 w-2/5 text-ellipsis overflow-auto'
                                >
                                  {metricReference}
                                </td>
                                <td
                                  className='px-1 py-1 w-2/5 text-ellipsis overflow-auto'
                                >
                                  {element.vmName}
                                </td>
                                <td
                                  className='pr-1 py-1 w-1/5 text-ellipsis overflow-auto'
                                >
                                  {roundToDecimalPlaces(
                                    element.value)}
                                </td>
                              </tr>
                            </tbody>
                          </table>
                        ))}
                      </td>
                    );
                  }

                  return (
                    <td key={metricKey} className='text-left px-4 text-ellipsies overflow-auto'>
                      {metricReference} - {t('tloTable.noMetricData')}
                    </td>
                  );
                },
                )}
              </tr>
            </tbody>
          </table>
        </td>
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
