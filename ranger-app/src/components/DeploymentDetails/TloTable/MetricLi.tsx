import React from 'react';
import {useTranslation} from 'react-i18next';
import {useGetMetricScoresQuery} from 'src/slices/apiSlice';
import {sortByVmNameAscending} from 'src/utils';

const MetricLi = (
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

  if (!scoreElements || scoreElements.length === 0) {
    return (
      <li>
        {t('error.noMetricData')}
      </li>
    );
  }

  const sortedScoreElements = scoreElements.slice().sort(sortByVmNameAscending);

  return (
    <>
      {sortedScoreElements.map(element => (
        <li key={element.id}>
          {metricName} - {element.vmName}: {Math
            .round(element.value * 100) / 100} points
        </li>
      ))}
    </>
  );
};

export default MetricLi;
