import React from 'react';
import type {ChartData, ChartDataset} from 'chart.js';
import {
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
import {Colors} from '@blueprintjs/core';
import zoomPlugin from 'chartjs-plugin-zoom';
import {type Score} from 'src/models/score';
import {
  defaultColors,
  groupByMetricNameAndVmName,
  roundToDecimalPlaces,
} from 'src/utils';
// eslint-disable-next-line import/no-unassigned-import
import 'chartjs-adapter-moment';
import {useTranslation} from 'react-i18next';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {sortByProperty} from 'sort-by-property';

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
      if (Object.prototype.hasOwnProperty.call(scoresByMetric, metricName)
      ) {
        const baseDataset: ChartDataset<'line'> = {
          type: 'line',
          label: metricName,
          tension: 0.3,
          borderColor: defaultColors,
          backgroundColor: defaultColors,
          pointBackgroundColor: Colors.WHITE,
          pointBorderColor: Colors.GRAY3,
          borderWidth: 1,
          parsing: false,
          fill: false,
          pointRadius: 1.5,
          data: [],
        };

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
    const minZoomRangeMillis = 60 * 1000;
    return (
      <Line
        data={intoGraphData(groupedScores)}
        options={{
          showLine: true,
          animation: false,
          parsing: false,
          interaction: {
            mode: 'point',
            axis: 'x',
            intersect: false,
          },
          indexAxis: 'x',
          plugins: {
            tooltip: {
              displayColors: false,
            },

            decimation: {
              enabled: true,
              algorithm: 'lttb',
              threshold: 100,
              samples: 100,
            },

            title: {
              display: true,
              text: chartTitle,
            },
            zoom: {
              pan: {
                enabled: true,
                mode: 'xy',
              },
              limits: {
                x: {
                  minRange: minZoomRangeMillis,
                  min: Date.parse(scenario?.start),
                  max: Date.parse(scenario?.end),
                },
                y: {
                  min: 'original',
                  max: 'original',
                },
              },
              zoom: {
                wheel: {
                  enabled: true,
                  speed: 0.2,
                },
                pinch: {
                  enabled: true,
                },
                mode: 'x',
              },
            },
          },
          responsive: true,
          scales: {
            y: {
              title: {
                display: true,
                text: yAxisTitle,
              },
              min: 0,
            },
            x: {
              title: {
                display: true,
                text: xAxisTitle,
              },
              min: Date.parse(scenario?.start),
              max: Date.parse(scenario?.end),
              ticks: {
                source: 'auto',
              },
              type: 'time',
              time: {
                displayFormats: {
                  hour: 'HH:mm',
                  minute: 'HH:mm',
                  second: 'HH:mm:ss',
                },
              },
            },
          },
        }}/>
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
