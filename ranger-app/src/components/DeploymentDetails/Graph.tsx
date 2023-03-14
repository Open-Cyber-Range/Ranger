import type React from 'react';
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
  useGetDeploymentScoresQuery,
} from 'src/slices/apiSlice';
import styled from 'styled-components';
import {Colors} from '@blueprintjs/core';
import {type ScoreElement} from 'src/models/scoreElement';
import {
  defaultColors,
  groupByMetricNameAndVmName,
  roundToDecimalPlaces,
  sortByCreatedAtAscending,
} from 'src/utils';
// eslint-disable-next-line import/no-unassigned-import
import 'chartjs-adapter-moment';
import {useTranslation} from 'react-i18next';
import {skipToken} from '@reduxjs/toolkit/dist/query';

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
);

const FallbackTextWrapper = styled.div`
  display: flex;
  justify-content: center;
  align-self: center;
  margin: 2px;
  margin-top: auto;
  margin-bottom: 1rem;
  color: ${Colors.GRAY3};
`;

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
  const {data: scoreElements} = useGetDeploymentScoresQuery(queryArguments);
  const {data: deployment} = useGetDeploymentQuery(queryArguments);

  const intoGraphPoint = (scoreElement: ScoreElement) => ({
    x: Date.parse(scoreElement.createdAt),
    y: roundToDecimalPlaces(scoreElement.value),
  });

  function intoGraphData(
    scoreElementsMap: Record<string, ScoreElement[]>) {
    const graphData: ChartData<'line'> = {
      datasets: [],
    };

    for (const metricName in scoreElementsMap) {
      if (Object.prototype.hasOwnProperty.call(scoreElementsMap, metricName)
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

        for (const scoreElement of scoreElementsMap[metricName]
          .sort(sortByCreatedAtAscending)
        ) {
          const graphPoint = intoGraphPoint(scoreElement);
          (baseDataset.data).push(graphPoint);
        }

        graphData.datasets.push(baseDataset);
      }
    }

    return graphData;
  }

  if (deployment && scoreElements && scoreElements.length > 0) {
    return (
      <Line
        data={intoGraphData(groupByMetricNameAndVmName(scoreElements))}
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
              min: deployment.startTime,
              max: deployment.endTime,
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
    <FallbackTextWrapper>
      {t('chart.scoring.noScoreData')}
    </FallbackTextWrapper>
  );
};

export default DeploymentDetailsGraph;
