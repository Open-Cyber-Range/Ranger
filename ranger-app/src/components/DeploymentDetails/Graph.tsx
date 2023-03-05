import type React from 'react';
import type {ChartData, ChartDataset} from 'chart.js';
import {
  Decimation,
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  BarElement,
  Title,
  Tooltip,
  Legend,
  PointElement,
  LineElement,
  TimeScale,
} from 'chart.js';
import {Line} from 'react-chartjs-2';
import {
  useGetDeploymentScoresQuery,
  useGetDeploymentStartTimeandEndTimeQuery,
} from 'src/slices/apiSlice';
import styled from 'styled-components';
import type {ScoreElement} from 'src/models/tlo';
import {
  defaultColors,
  definedOrSkipToken,
  groupByMetricNameAndVmName,
  parseStringDateToMillis,
  roundToDecimalPlaces,
  sortByCreatedAtAscending,
} from 'src/utils';
// eslint-disable-next-line import/no-unassigned-import
import 'chartjs-adapter-moment';
import {useTranslation} from 'react-i18next';

ChartJS.register(
  CategoryScale,
  TimeScale,
  LinearScale,
  BarElement,
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
  color: #a6a3a3;
`;

const DeploymentDetailsGraph = ({exerciseId, deploymentId}:
{exerciseId: string | undefined;
  deploymentId: string | undefined;
}) => {
  const {t} = useTranslation();
  const xAxisTitle = t('chart.xAxisTitle');
  const yAxisTitle = t('chart.yAxisTitle');
  const chartTitle = t('chart.title');
  const {data: scoreElements} = useGetDeploymentScoresQuery(
    definedOrSkipToken(exerciseId, deploymentId));
  const {data: deploymentTimeRange} = useGetDeploymentStartTimeandEndTimeQuery(
    definedOrSkipToken(exerciseId, deploymentId));

  if (!scoreElements || scoreElements.length === 0) {
    return (
      <FallbackTextWrapper>
        {t('error.noScoreData')}
      </FallbackTextWrapper>
    );
  }

  const intoGraphPoint = (scoreElement: ScoreElement) => ({
    x: parseStringDateToMillis(scoreElement.createdAt),
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
          pointBackgroundColor: '#ffffff',
          pointBorderColor: '#808080',
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
            min: deploymentTimeRange?.startTime,
            max: deploymentTimeRange?.endTime,
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
};

export default DeploymentDetailsGraph;
