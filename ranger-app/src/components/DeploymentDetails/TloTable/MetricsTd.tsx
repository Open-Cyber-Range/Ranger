import React from 'react';
import MetricLi from './MetricLi';

const MetricsTd = ({exerciseId, deploymentId, tloName, metrics}:
{exerciseId: string;
  deploymentId: string;
  tloName: string;
  metrics: string[];
}) => (
  <td key={tloName}>
    {metrics.map(metricName => (
      <MetricLi
        key={metricName}
        exerciseId={exerciseId}
        tloName={tloName}
        deploymentId={deploymentId}
        metricName={metricName}/>
    ))}
  </td>
);

export default MetricsTd;
