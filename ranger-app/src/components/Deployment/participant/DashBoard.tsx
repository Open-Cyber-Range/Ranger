import React from 'react';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import EntitySelect from 'src/components/EntitySelect';
import {useParticipantGetDeploymentScenarioQuery} from 'src/slices/apiSlice';
import {useDispatch, useSelector} from 'react-redux';
import {selectedEntity, setSelectedEntity} from 'src/slices/userSlice';

const ParticipantDashBoard = ({
  exerciseId, deploymentId}: {exerciseId: string; deploymentId: string;
}) => {
  const {data: scenario} = useParticipantGetDeploymentScenarioQuery(exerciseId && deploymentId
    ? {exerciseId, deploymentId} : skipToken);
  const currentEntity = useSelector(selectedEntity);
  const dispatch = useDispatch();

  return (
    <div>
      <EntitySelect
        entities={scenario?.entities}
        selectedEntityKey={currentEntity}
        onChange={(selectedKey: string | undefined) => {
          dispatch(setSelectedEntity(selectedKey));
        }}
      />
    </div>
  );
};

export default ParticipantDashBoard;
