
import React from 'react';
import {useTranslation} from 'react-i18next';
import {type Score} from 'src/models/score';
import {findLatestScoresByVms, roundToDecimalPlaces} from 'src/utils';

const MetricInfo = (
  {metricName, scores}:
  {metricName: string; scores: Score[] | undefined}) => {
  const {t} = useTranslation();

  if (scores && scores.length > 0) {
    const latestScoresByVm = findLatestScoresByVms(scores);
    latestScoresByVm.sort((a, b) => a.vmName.localeCompare(b.vmName));

    return (
      <>
        {latestScoresByVm.map(element => (
          <li key={element.id}>
            {metricName} - {element.vmName}: {roundToDecimalPlaces(
              element.value)} {t('tloTable.points')}
          </li>
        ))}
      </>
    );
  }

  return (
    <li>
      {t('tloTable.noMetricData')}
    </li>
  );
};

export default MetricInfo;
