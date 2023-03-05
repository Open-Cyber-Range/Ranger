import React from 'react';
import {useTranslation} from 'react-i18next';
import {useGetMetricScoresQuery} from 'src/slices/apiSlice';
import {roundToDecimalPlaces} from 'src/utils';

const MetricInfo = (
  {exerciseId, deploymentId, tloName, metricName}:
  {exerciseId: string;
    tloName: string;
    deploymentId: string;
    metricName: string;
  }) => {
  const {t} = useTranslation();
  const {data: scoreElements} = useGetMetricScoresQuery({
    exerciseId,
    deploymentId,
    tloName,
    metricName,
  });

  if (scoreElements && scoreElements.length > 0) {
    const sortedScoreElements = scoreElements.slice()
      .sort((a, b) => a.vmName.localeCompare(b.vmName));

    return (
      <>
        {sortedScoreElements.map(element => (
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
