import React, {useMemo} from 'react';
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
import zoomPlugin from 'chartjs-plugin-zoom';
import {type Score} from 'src/models/score';
import {
  getMetricReferencesByRole,
  groupByMetricNameAndVmName,
} from 'src/utils';
// eslint-disable-next-line import/no-unassigned-import
import 'chartjs-adapter-luxon';
import {useTranslation} from 'react-i18next';
import {getLineChartOptions, scoresIntoGraphData} from 'src/utils/graph';
import {
  type Entity,
  type TrainingLearningObjective,
  type Evaluation,
  type Metric,
} from 'src/models/scenario';

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

const DeploymentDetailsGraph = (
  {entities, tlos, evaluations, metrics, scenarioStart, scenarioEnd, scores, colorsByRole}:
  {
    entities: Record<string, Entity>;
    tlos: Record<string, TrainingLearningObjective>;
    evaluations: Record<string, Evaluation>;
    metrics: Record<string, Metric>;
    scenarioStart: string;
    scenarioEnd: string;
    scores: Score[];
    colorsByRole?: boolean;
  }) => {
  const {t} = useTranslation();
  const xAxisTitle = t('chart.scoring.xAxisTitle');
  const yAxisTitle = t('chart.scoring.yAxisTitle');
  const chartTitle = t('chart.scoring.title');

  const metricReferencesByRole = getMetricReferencesByRole(entities, tlos, evaluations, metrics);
  const minLimit = Date.parse(scenarioStart);
  const maxLimit = Date.parse(scenarioEnd);

  const options = useMemo(() => getLineChartOptions({
    minLimit,
    maxLimit,
    chartTitle,
    xAxisTitle,
    yAxisTitle},
  ), [chartTitle, xAxisTitle, yAxisTitle, minLimit, maxLimit]);

  if (scores.length > 0) {
    const groupedScores = groupByMetricNameAndVmName(scores);

    return (
      <Line
        data={scoresIntoGraphData(groupedScores, metricReferencesByRole, colorsByRole)}
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
