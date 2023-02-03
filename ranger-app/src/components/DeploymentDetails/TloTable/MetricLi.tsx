import React from 'react';
import {useGetMetricScoresQuery} from 'src/slices/apiSlice';

const MetricLi = (
  {exerciseId, deploymentId, tloName, metricName}:
  {exerciseId: string;
    deploymentId: string;
    tloName: string;
    metricName: string;
  }) => {
  const {data: scoreElements} = useGetMetricScoresQuery(
    {exerciseId,
      deploymentId,
      tloName,
      metricName,
    });

  if (!scoreElements || scoreElements.length === 0) {
    return (
      <li>
        No metric scores to display
      </li>
    );
  }

  const currentScoreElements = scoreElements
    .filter(score => score.metricName === metricName);

  const parsedMetricDates = currentScoreElements.map(scoreElement =>
    Date.parse(scoreElement.createdAt.replace(/-/g, '/')));

  const latestMetricDate = Math.max(...parsedMetricDates);
  const latestScoreEelement = currentScoreElements
    .find(metric => Date.parse(metric.createdAt) === latestMetricDate);

  return (
    <li key={latestScoreEelement?.id}>
      {latestScoreEelement?.metricName ?? metricName}{' - '}
      {latestScoreEelement ? Math
        .round(latestScoreEelement.value * 100 * 100) / 100 : 'No'}{' points'}
    </li>
  );
};

export default MetricLi;
