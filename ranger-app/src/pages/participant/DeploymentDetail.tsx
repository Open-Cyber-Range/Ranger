import React, {useState} from 'react';
import {useParams} from 'react-router-dom';
import type {DeploymentDetailRouteParameters} from 'src/models/routes';
import {useTranslation} from 'react-i18next';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {
  useParticipantGetDeploymentQuery,
  useParticipantGetDeploymentScenarioQuery,
} from 'src/slices/apiSlice';
import SideBar from 'src/components/Exercise/participant/SideBar';
import ParticipantDeploymentGraph from 'src/components/Scoring/participant/Graph';
import {H2} from '@blueprintjs/core';
import EntitySelect from 'src/components/EntitySelect';
import {type Entity} from 'src/models/scenario';
import TloTable from 'src/components/Scoring/participant/TloTable';

const ParticipantDeploymentDetail = () => {
  const {t} = useTranslation();
  const {exerciseId, deploymentId} = useParams<DeploymentDetailRouteParameters>();
  const {data: deployment} = useParticipantGetDeploymentQuery(
    exerciseId && deploymentId ? {exerciseId, deploymentId} : skipToken,
  );
  const {data: scenario} = useParticipantGetDeploymentScenarioQuery(exerciseId && deploymentId
    ? {exerciseId, deploymentId} : skipToken);

  const [selectedEntityKey, setSelectedEntityKey] = useState<string | undefined>(undefined);

  const handleEntityChange = (selectedKey: string | undefined) => {
    setSelectedEntityKey(selectedKey);
  };

  let selectedEntity: Entity | undefined;
  if (scenario?.entities) {
    selectedEntity = scenario?.entities[selectedEntityKey ?? ''];
  }

  if (exerciseId && deploymentId) {
    return (
      <SideBar renderMainContent={() => (
        <>
          <H2>{deployment?.name}</H2>

          <EntitySelect
            entities={scenario?.entities}
            selectedEntityKey={selectedEntityKey}
            onChange={handleEntityChange}
          />
          <br/>
          <ParticipantDeploymentGraph
            exerciseId={exerciseId}
            deploymentId={deploymentId}
          />

          <TloTable
            exerciseId={exerciseId}
            deploymentId={deploymentId}
            tloMap={scenario?.tlos}
            selectedEntity={selectedEntity}
          />
        </>
      )}
      />
    );
  }

  return (
    <div className='flex justify-center align-center m-2 mt-10 mb-auto text-gray-400'>
      {t('exercises.noDeploymentInfo')}
    </div>
  );
};

export default ParticipantDeploymentDetail;
