
import React from 'react';
import {useTranslation} from 'react-i18next';
import {type ScoreElement} from 'src/models/scoreElement';
import {findLatestScoreElementsByVms, roundToDecimalPlaces} from 'src/utils';

const MetricInfo = (
  {metricName, scoreElements}:
  {metricName: string; scoreElements: ScoreElement[] | undefined}) => {
  const {t} = useTranslation();

  if (scoreElements && scoreElements.length > 0) {
    const latestScoresByVm = findLatestScoreElementsByVms(scoreElements);
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
