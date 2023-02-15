import React from 'react';
import {useTranslation} from 'react-i18next';
import {useGetMetricScoresQuery} from 'src/slices/apiSlice';
import {parseStringDateToMillis} from 'src/utils';

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

  const currentScoreElements = scoreElements
    .filter(score => score.metricName === metricName);

  const parsedMetricDates = currentScoreElements.map(scoreElement =>
    parseStringDateToMillis(scoreElement.createdAt));
  const latestMetricDate = Math.max(...parsedMetricDates);
  const latestScoreEelement = currentScoreElements
    .find(metric => Date.parse(metric.createdAt) === latestMetricDate);

  return (
    <li key={latestScoreEelement?.metricName}>
      {latestScoreEelement?.metricName ?? metricName}{' - '}
      {latestScoreEelement ? Math
        .round(latestScoreEelement.value * 100) / 100 : 'No'}{' points'}
    </li>
  );
};

export default MetricLi;
