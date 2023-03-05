import React from 'react';
import {useTranslation} from 'react-i18next';
import {useGetMetricScoresQuery} from 'src/slices/apiSlice';
import {roundToDecimalPlaces, sortByVmNameAscending} from 'src/utils';

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
      .sort(sortByVmNameAscending);

    return (
      <>
        {sortedScoreElements.map(element => (
          <li key={element.id}>
            {metricName} - {element.vmName}: {roundToDecimalPlaces(
              element.value)} points
          </li>
        ))}
      </>
    );
  }

  return (
    <li>
      {t('error.noMetricData')}
    </li>
  );
};

export default MetricInfo;
