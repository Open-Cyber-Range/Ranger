
import React from 'react';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {useParticipantGetDeploymentScenarioQuery} from 'src/slices/apiSlice';
import {getMetricsByEntityKey} from 'src/utils';
import {type Metric, MetricType} from 'src/models/scenario';

const AddMetricForm = ({exerciseId, deploymentId, selectedEntity}:
{exerciseId: string;
  deploymentId: string;
  selectedEntity: string;
}) => {
//   Const {t} = useTranslation();
  const entitySelector = selectedEntity;
  const queryParameters
= exerciseId && deploymentId && entitySelector
  ? {exerciseId, deploymentId, entitySelector} : skipToken;
  const {data: scenario} = useParticipantGetDeploymentScenarioQuery(queryParameters);
  // Const manual_metrics: ManualMetric[] = []; // Get existing ManualMetrics from api

  if (scenario?.metrics) {
    const entityMetrics = getMetricsByEntityKey(selectedEntity, scenario);
    const entityManualMetrics = Object.keys(entityMetrics)
      .filter(key => entityMetrics[key].type === MetricType.Manual)
      .reduce<Record<string, Metric>>((accumulator, key) => {
      accumulator[key] = entityMetrics[key];
      return accumulator;
    }, {});

    // Compare manual_metrics to entityManualMetrics by entity_selector and metric_key
    // remove duplicates from entityManualMetrics
    // set manual_metrics into a PUT form, entityManualMetrics into a POST form
    // both should be in the same AccordionGroup

    if (entityManualMetrics) {
      return (
        <div>
          ManualMetric forms here
        </div>
      );
    }

    return (
      <div className='flex flex-col mt-2'>
        herro {selectedEntity}
      </div>
    );
  }

  return null;
};

export default AddMetricForm;
