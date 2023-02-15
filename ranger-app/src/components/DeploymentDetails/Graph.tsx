import type React from 'react';
import type {ChartData, ChartDataset} from 'chart.js';
import {
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
import {useGetDeploymentScoresQuery} from 'src/slices/apiSlice';
import styled from 'styled-components';
import type {ScoreElement} from 'src/models/tlo';
import {
  definedOrSkipToken,
  generateRandomColor,
  groupBy,
  parseStringDateToMillis,
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
  const {data: scoreElements} = useGetDeploymentScoresQuery(
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
    y: Math.round(scoreElement.value * 100) / 100,
  });

  function intoGraphData(
    scoreElementsMap: Record<string, ScoreElement[]>) {
    const graphData: ChartData<'line'> = {
      datasets: [],
    };

    for (const metricName in scoreElementsMap) {
      if (Object.prototype.hasOwnProperty.call(scoreElementsMap, metricName)
      ) {
        const randomColor = generateRandomColor();
        const baseDataset: ChartDataset<'line'> = {
          type: 'line',
          label: metricName,
          tension: 0.3,
          borderColor: randomColor,
          backgroundColor: randomColor,
          borderWidth: 1,
          fill: false,
          pointRadius: 1,
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

  const groupedScoreElements = groupBy(scoreElements, 'metricName');

  const graphData = intoGraphData(groupedScoreElements);

  return (
    <Line
      data={graphData}
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
          decimation: {
            enabled: true,
            algorithm: 'min-max',
            threshold: 1,
          },

          title: {
            display: true,
            text: 'Score',
          },
        },
        responsive: true,
        clip: false,
        scales: {
          y: {
            min: 0,
          },
          x: {
            ticks: {
              source: 'auto',
              maxRotation: 0,
            },
            type: 'time',
          },
        },
      }}/>
  );
};

export default DeploymentDetailsGraph;
