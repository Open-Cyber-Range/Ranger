import React from 'react';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {useParticipantGetDeploymentScenarioQuery} from 'src/slices/apiSlice';
import {useSelector} from 'react-redux';
import AddMetricForm from 'src/components/Scoring/participant/AddMetric';
import {selectedEntity} from 'src/slices/userSlice';

const ManualMetrics = ({
  exerciseId, deploymentId}:
{exerciseId: string; deploymentId: string;
}) => {
  // Const {t} = useTranslation();
  const entitySelector = useSelector(selectedEntity);
  const scenarioQueryArguments = exerciseId && deploymentId && entitySelector
    ? {exerciseId, deploymentId, entitySelector} : skipToken;
  const {data: scenario} = useParticipantGetDeploymentScenarioQuery(scenarioQueryArguments);
  const metrics = scenario?.metrics;

  if (metrics && entitySelector) {
    return (
      <div>
        <AddMetricForm
          exerciseId={exerciseId}
          deploymentId={deploymentId}
          selectedEntity={entitySelector}/>
      </div>
    );
  }

  return (
    <div className='
    flex justify-center align-center m-2 mt-auto mb-4 text-gray-400'
    >
      No metrics
    </div>
  );
};

export default ManualMetrics;
