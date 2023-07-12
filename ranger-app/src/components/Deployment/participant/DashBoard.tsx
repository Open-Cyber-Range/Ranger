import {skipToken} from '@reduxjs/toolkit/dist/query';
import React, {useState} from 'react';
import EntitySelect from 'src/components/EntitySelect';
import {useParticipantGetDeploymentScenarioQuery} from 'src/slices/apiSlice';

const ParticipantDashBoard = ({
  exerciseId, deploymentId}: {exerciseId: string; deploymentId: string;
}) => {
  const {data: scenario} = useParticipantGetDeploymentScenarioQuery(exerciseId && deploymentId
    ? {exerciseId, deploymentId} : skipToken);
  const [selectedEntityKey, setSelectedEntityKey] = useState<string | undefined>(undefined);

  return (
    <div>
      <EntitySelect
        entities={scenario?.entities}
        selectedEntityKey={selectedEntityKey}
        onChange={(selectedKey: string | undefined) => {
          setSelectedEntityKey(selectedKey);
        }}
      />
    </div>
  );
};

export default ParticipantDashBoard;
