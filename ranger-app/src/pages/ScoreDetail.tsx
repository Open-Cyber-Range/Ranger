import React from 'react';
import {useParams} from 'react-router-dom';
import type {DeploymentDetailScoresRouteParameters} from 'src/models/routes';
import {useTranslation} from 'react-i18next';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {
  useAdminGetDeploymentScenarioQuery,
  useAdminGetDeploymentScoresQuery,
} from 'src/slices/apiSlice';
import useExerciseStreaming from 'src/hooks/useExerciseStreaming';
import {ExerciseRoleOrder} from 'src/models/scenario';
import {
  flattenEntities,
  getTloKeysByRole,
  getUniqueRoles,
  groupTloMapsByRoles,
  tryIntoScoringMetadata,
} from 'src/utils';
import TloTable from 'src/components/Scoring/TloTable';
import DeploymentDetailsGraph from 'src/components/Scoring/Graph';
import PageHolder from 'src/components/PageHolder';
import SideBar from 'src/components/Exercise/SideBar';
import ScoreTag from 'src/components/Scoring/ScoreTag';

const ScoreDetail = () => {
  const {t} = useTranslation();
  const {exerciseId, deploymentId, role} = useParams<DeploymentDetailScoresRouteParameters>();
  useExerciseStreaming(exerciseId);
  const queryArguments = exerciseId && deploymentId ? {exerciseId, deploymentId} : skipToken;
  const {data: scenario} = useAdminGetDeploymentScenarioQuery(queryArguments);
  const {data: scores} = useAdminGetDeploymentScoresQuery(queryArguments);
  const scoringData = tryIntoScoringMetadata(scenario);

  if (deploymentId && exerciseId && role && scoringData) {
    const flattenedEntities = flattenEntities(scoringData.entities);
    const tloKeysByRole = getTloKeysByRole(flattenedEntities, role);
    const roles = getUniqueRoles(flattenedEntities)
      .sort((a, b) => ExerciseRoleOrder[a] - ExerciseRoleOrder[b]);
    const tlosByRole = groupTloMapsByRoles(flattenedEntities, scoringData.tlos, roles);

    const metricKeysByTloKeys = tloKeysByRole.map(tloKey => scoringData.tlos[tloKey])
      .map(tlo => tlo.evaluation)
      .map(evaluationKey => scoringData.evaluations[evaluationKey])
      .flatMap(evaluation => evaluation.metrics);
    const metricReferences = new Set(metricKeysByTloKeys
      .map(metricKey => scoringData.metrics[metricKey]?.name ?? metricKey));
    const filteredScores = scores?.filter(score => metricReferences.has(score.metricName));

    return (
      <SideBar renderMainContent={() => (
        <PageHolder>
          <div>
            <div className='flex flex-col mt-6 text-center font-bold'>
              <ScoreTag
                key={role}
                large
                exerciseId={exerciseId}
                deploymentId={deploymentId}
                role={role}
              />
            </div>
            <DeploymentDetailsGraph
              scoringData={scoringData}
              scores={filteredScores}
            />
            <TloTable
              scoringData={scoringData}
              scores={filteredScores}
              tloMap={tlosByRole[role]}
            />
          </div>
        </PageHolder>
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

export default ScoreDetail;
