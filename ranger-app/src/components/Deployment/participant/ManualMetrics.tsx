import React from 'react';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {
  useParticipantGetDeploymentScenarioQuery,
  useParticipantGetMetricsQuery,
} from 'src/slices/apiSlice';
import {useSelector} from 'react-redux';
import {selectedEntity} from 'src/slices/userSlice';
import Accordion from 'src/components/Accordion';
import AccordionGroup from 'src/components/AccordionGroup';
import {type Metric as ScenarioMetric, MetricType} from 'src/models/scenario';
import {flattenEntities, getMetricsByEntityKey} from 'src/utils';
import AddNewMetric from 'src/components/Scoring/participant/AddMetric';
import UpdateMetric from 'src/components/Scoring/participant/UpdateMetric';
import {useTranslation} from 'react-i18next';
import {Callout} from '@blueprintjs/core';

const ManualMetrics = ({
  exerciseId, deploymentId}:
{exerciseId: string; deploymentId: string;
}) => {
  const {t} = useTranslation();
  const entitySelector = useSelector(selectedEntity);
  const participantQueryArgs = exerciseId && deploymentId && entitySelector
    ? {exerciseId, deploymentId, entitySelector} : skipToken;

  const {data: scenario} = useParticipantGetDeploymentScenarioQuery(participantQueryArgs);
  let {data: existingManualMetrics} = useParticipantGetMetricsQuery(participantQueryArgs);
  const entities = scenario?.entities ?? {};
  const entity = flattenEntities(entities)[entitySelector ?? ''];
  const entityRole = entity?.role;

  if (existingManualMetrics) {
    existingManualMetrics = existingManualMetrics
      .filter(metric => metric.entitySelector === entitySelector);
  }

  if (entitySelector && scenario?.metrics && existingManualMetrics && entityRole) {
    const manualMetricNames = new Set(existingManualMetrics.map(metric => metric.name));
    const entityScenarioMetrics = getMetricsByEntityKey(entitySelector, scenario);

    const entityScenarioManualMetrics = Object.entries(entityScenarioMetrics)
      .filter(([key, scenarioMetric]) =>
        scenarioMetric.type === MetricType.Manual
        && !(manualMetricNames.has(key) || manualMetricNames.has(scenarioMetric.name ?? '')))
      .reduce<Record<string, ScenarioMetric>>((accumulator, [key, scenarioMetric]) => {
      accumulator[key] = scenarioMetric;
      return accumulator;
    }, {});

    if (Object.keys(entityScenarioManualMetrics).length > 0
      || (existingManualMetrics && existingManualMetrics.length > 0)) {
      return (
        <div>
          <AccordionGroup>
            {Object.entries(entityScenarioManualMetrics).map(([key, metric]) => (
              <Accordion
                key={key}
                title={metric.name ?? key}
                className='mb-4 p-2 border-2 border-slate-300 shadow-md '
              >
                <AddNewMetric
                  exerciseId={exerciseId}
                  deploymentId={deploymentId}
                  newManualMetric={{exerciseId,
                    deploymentId, entitySelector, metricKey: key, role: entityRole}}
                  metricHasArtifact={metric.artifact ?? false}
                />
              </Accordion>
            ))}
            {Object.entries(existingManualMetrics).map(([key, metric]) => {
              const scenarioMetric = scenario?.metrics?.[metric.name]
              ?? Object.values(scenario?.metrics ?? {})
                .find(scenarioMetric => scenarioMetric.name === metric.name);
              return (
                <Accordion
                  key={key}
                  title={metric.name}
                  className='mb-4 p-2 border-2 border-slate-300 shadow-md '
                >
                  <UpdateMetric
                    exerciseId={exerciseId}
                    deploymentId={deploymentId}
                    manualMetric={metric}
                    metricHasArtifact={scenarioMetric?.artifact ?? false}
                  />
                </Accordion>
              );
            })}
          </AccordionGroup>
        </div>
      );
    }
  }

  return (
    <Callout title={t('metricScoring.errors.noMetrics') ?? ''}/>
  );
};

export default ManualMetrics;
