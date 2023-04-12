import React from 'react';
import {
  type ChartData,
  Decimation,
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  Title,
  Tooltip,
  Legend,
  PointElement,
  LineElement,
  TimeScale,
} from 'chart.js';
import {Line} from 'react-chartjs-2';
import {
  useGetDeploymentQuery,
  useGetDeploymentScenarioQuery,
  useGetDeploymentScoresQuery,
} from 'src/slices/apiSlice';
import zoomPlugin from 'chartjs-plugin-zoom';
import {type Score} from 'src/models/score';
import {groupByMetricNameAndVmName, roundToDecimalPlaces} from 'src/utils';
// eslint-disable-next-line import/no-unassigned-import
import 'chartjs-adapter-moment';
import {useTranslation} from 'react-i18next';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {sortByProperty} from 'sort-by-property';
import {LINE_DATASET_TEMPLATE} from 'src/constants';
import cloneDeep from 'lodash.clonedeep';
import {getLineChartOptions} from 'src/utils/graph';

ChartJS.register(
  CategoryScale,
  TimeScale,
  LinearScale,
  Title,
  Tooltip,
  Legend,
  PointElement,
  LineElement,
  Decimation,
  zoomPlugin,
);

const DeploymentDetailsGraph = ({exerciseId, deploymentId}:
{exerciseId: string | undefined;
  deploymentId: string | undefined;
}) => {
  const {t} = useTranslation();
  const xAxisTitle = t('chart.scoring.xAxisTitle');
  const yAxisTitle = t('chart.scoring.yAxisTitle');
  const chartTitle = t('chart.scoring.title');
  const queryArguments = exerciseId && deploymentId
    ? {exerciseId, deploymentId} : skipToken;
  const {data: scores} = useGetDeploymentScoresQuery(queryArguments);
  const {data: deployment} = useGetDeploymentQuery(queryArguments);
  const {data: scenario} = useGetDeploymentScenarioQuery(queryArguments);

  const intoGraphPoint = (score: Score) => ({
    x: Date.parse(score.timestamp),
    y: roundToDecimalPlaces(score.value),
  });

  function intoGraphData(
    scoresByMetric: Record<string, Score[]>) {
    const graphData: ChartData<'line'> = {
      datasets: [],
    };

    for (const metricName in scoresByMetric) {
      if (scoresByMetric[metricName]) {
        const baseDataset = cloneDeep(LINE_DATASET_TEMPLATE);
        baseDataset.label = metricName;

        for (const score of scoresByMetric[metricName]
          .sort(sortByProperty('timestamp', 'asc'))
        ) {
          const graphPoint = intoGraphPoint(score);
          (baseDataset.data).push(graphPoint);
        }

        graphData.datasets.push(baseDataset);
      }
    }

    return graphData;
  }

  if (deployment && scenario && scores && scores.length > 0) {
    const groupedScores = groupByMetricNameAndVmName(scores);
    const options = getLineChartOptions({
      minLimit: Date.parse(scenario?.start),
      maxLimit: Date.parse(scenario?.end),
      chartTitle,
      xAxisTitle,
      yAxisTitle},
    );

    return (
      <Line
        data={intoGraphData(groupedScores)}
        options={options}/>
    );
  }

  return (
    <div className='
    flex justify-center align-center m-2 mt-auto mb-4 text-gray-400'
    >
      {t('chart.scoring.noScoreData')}
    </div>
  );
};

export default DeploymentDetailsGraph;
