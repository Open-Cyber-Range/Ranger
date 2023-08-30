import React, {useMemo} from 'react';
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
import {DateTime} from 'luxon';
import {
  useAdminGetDeploymentQuery,
  useAdminGetDeploymentScenarioQuery,
} from 'src/slices/apiSlice';
import zoomPlugin from 'chartjs-plugin-zoom';
import {type Score} from 'src/models/score';
import {
  defaultColors,
  lineColorsByRole,
  flattenEntities,
  groupByMetricNameAndVmName,
  roundToDecimalPlaces,
} from 'src/utils';
// eslint-disable-next-line import/no-unassigned-import
import 'chartjs-adapter-luxon';
import {useTranslation} from 'react-i18next';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {sortByProperty} from 'sort-by-property';
import {LINE_DATASET_TEMPLATE} from 'src/constants';
import cloneDeep from 'lodash.clonedeep';
import {getLineChartOptions} from 'src/utils/graph';
import {type ExerciseRole} from 'src/models/scenario';

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

const DeploymentDetailsGraph = ({exerciseId, deploymentId, scores, colorsByRole}:
{exerciseId: string | undefined;
  deploymentId: string | undefined;
  scores: Score[] | undefined;
  colorsByRole?: boolean;
}) => {
  const {t} = useTranslation();
  const xAxisTitle = t('chart.scoring.xAxisTitle');
  const yAxisTitle = t('chart.scoring.yAxisTitle');
  const chartTitle = t('chart.scoring.title');
  const queryArguments = exerciseId && deploymentId
    ? {exerciseId, deploymentId} : skipToken;
  const {data: deployment} = useAdminGetDeploymentQuery(queryArguments);
  const {data: scenario} = useAdminGetDeploymentScenarioQuery(queryArguments);
  const flattenedEntities = flattenEntities(scenario?.entities ?? {});

  const tlos = scenario?.tlos ?? {};
  const evaluations = scenario?.evaluations ?? {};
  const metrics = scenario?.metrics ?? {};

  const metricReferencesByRole = Object.values(flattenedEntities)
    .reduce<Record<ExerciseRole, Set<string>>>((acc, entity) => {
    const role = entity.role;
    const entityTlos = entity.tlos;
    if (role && entityTlos) {
      const metricReferences = entityTlos.map(tloKey => tlos[tloKey])
        .map(tlo => tlo.evaluation)
        .map(evaluationKey => evaluations[evaluationKey])
        .flatMap(evaluation => evaluation.metrics)
        .map(metricKey => metrics[metricKey].name ?? metricKey);

      for (const metricReference of metricReferences) {
        acc[role].add(metricReference);
      }
    }

    return acc;
  }, {
    Blue: new Set(),
    Green: new Set(),
    Red: new Set(),
    White: new Set(),
  });

  const getLineColorByMetricReference = (metricReference: string) => {
    const roles = Object.keys(metricReferencesByRole) as ExerciseRole[];
    const metricRole = roles.find(role => metricReferencesByRole[role].has(metricReference));

    return metricRole ? lineColorsByRole[metricRole] ?? defaultColors : defaultColors;
  };

  const intoGraphPoint = (score: Score) => ({
    x: DateTime.fromISO(score.timestamp, {zone: 'utc'}).toMillis(),
    y: roundToDecimalPlaces(score.value),
  });

  function intoGraphData(
    scoresByMetrics: Record<string, Score[]>) {
    const graphData: ChartData<'line'> = {
      datasets: [],
    };

    for (const metricLineLabel in scoresByMetrics) {
      if (scoresByMetrics[metricLineLabel]) {
        const baseDataset = cloneDeep(LINE_DATASET_TEMPLATE);
        baseDataset.label = metricLineLabel;
        if (colorsByRole) {
          const metricName = scoresByMetrics[metricLineLabel][0].metricName;
          baseDataset.borderColor = getLineColorByMetricReference(metricName);
          baseDataset.backgroundColor = getLineColorByMetricReference(metricName);
        }

        for (const score of scoresByMetrics[metricLineLabel]
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

  let minLimit: number | undefined;
  let maxLimit: number | undefined;

  if (scenario) {
    minLimit = Date.parse(scenario.start);
    maxLimit = Date.parse(scenario.end);
  }

  const options = useMemo(() => getLineChartOptions({
    minLimit,
    maxLimit,
    chartTitle,
    xAxisTitle,
    yAxisTitle},
  ), [chartTitle, xAxisTitle, yAxisTitle, minLimit, maxLimit]);

  if (deployment && scenario && scores && scores.length > 0) {
    const groupedScores = groupByMetricNameAndVmName(scores);

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
