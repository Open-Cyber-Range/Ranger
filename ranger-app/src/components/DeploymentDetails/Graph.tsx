import type React from 'react';
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
  sortByCreatedAtAscending,
} from 'src/utils';
// eslint-disable-next-line import/no-unassigned-import
import 'chartjs-adapter-moment';
import type {GraphData, GraphDataset, GraphPoint} from 'src/models/scoreGraph';

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
  const {data: scoreElements} = useGetDeploymentScoresQuery(
    definedOrSkipToken(exerciseId, deploymentId));

  if (!scoreElements || scoreElements.length === 0) {
    return (
      <FallbackTextWrapper>
        No score data to display graph
      </FallbackTextWrapper>
    );
  }

  const intoGraphPoint = (scoreElement: ScoreElement) => (
    {
      x: Date.parse(scoreElement.createdAt.replace(/-/g, '/')),
      y: Math.round(scoreElement.value * 100 * 100) / 100,
    }
  );

  function intoGraphData(
    scoreElementsMap: Record<string, ScoreElement[]>) {
    const graphData: GraphData = {
      datasets: [],
    };

    for (const metricName in scoreElementsMap) {
      if (Object.prototype.hasOwnProperty.call(scoreElementsMap, metricName)
      ) {
        const ramdomColor = generateRandomColor();
        const baseDataset: GraphDataset = {
          label: metricName,
          tension: 0.3,
          borderColor: ramdomColor,
          backgroundColor: ramdomColor,
          data: [],
        };

        for (const scoreElement of scoreElementsMap[metricName]
          .sort(sortByCreatedAtAscending)) {
          const graphPoint: GraphPoint = intoGraphPoint(scoreElement);
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
        plugins: {
          title: {
            display: true,
            text: 'Score',
          },
        },
        scales: {
          y: {
            min: 0,
            max: 100,
          },
          x: {
            type: 'time',
          },
        },
      }}/>
  );
};

export default DeploymentDetailsGraph;
