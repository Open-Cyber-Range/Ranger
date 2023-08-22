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
} from 'src/utils';
import TloTable from 'src/components/Scoring/TloTable';
import DeploymentDetailsGraph from 'src/components/Scoring/Graph';
import BackButton from 'src/components/BackButton';
import PageHolder from 'src/components/PageHolder';

const ScoreDetail = () => {
  const {t} = useTranslation();
  const {exerciseId, deploymentId, role} = useParams<DeploymentDetailScoresRouteParameters>();
  useExerciseStreaming(exerciseId);

  const queryArguments = exerciseId && deploymentId ? {exerciseId, deploymentId} : skipToken;
  const {data: scenario} = useAdminGetDeploymentScenarioQuery(queryArguments);
  const {data: scores} = useAdminGetDeploymentScoresQuery(queryArguments);

  const entities = scenario?.entities;
  const tlos = scenario?.tlos;
  const evaluations = scenario?.evaluations;
  const metrics = scenario?.metrics;

  if (deploymentId && exerciseId && tlos && entities && role && scores && evaluations && metrics) {
    const flattenedEntities = flattenEntities(entities);
    const tloKeysByRole = getTloKeysByRole(flattenedEntities, role);
    const roles = getUniqueRoles(flattenedEntities)
      .sort((a, b) => ExerciseRoleOrder[a] - ExerciseRoleOrder[b]);
    const tlosByRole = groupTloMapsByRoles(
      flattenedEntities, tlos, roles);

    const metricKeysByTloKeys = tloKeysByRole.map(tloKey => tlos[tloKey])
      .map(tlo => tlo.evaluation)
      .map(evaluationKey => evaluations[evaluationKey])
      .flatMap(evaluation => evaluation.metrics);
    const metricReferences = new Set(metricKeysByTloKeys
      .map(metricKey => metrics[metricKey]?.name ?? metricKey));
    const filteredScores = scores.filter(score => metricReferences.has(score.metricName));

    return (
      <PageHolder>
        <div>
          <BackButton/>
          <DeploymentDetailsGraph
            exerciseId={exerciseId}
            deploymentId={deploymentId}
            scores={filteredScores}
          />
          <TloTable
            exerciseId={exerciseId}
            deploymentId={deploymentId}
            tloMap={tlosByRole[role] ?? {}}
          />
        </div>
      </PageHolder>

    );
  }

  return (
    <div className='flex justify-center align-center m-2 mt-10 mb-auto text-gray-400'>
      {t('exercises.noDeploymentInfo')}
    </div>
  );
};

export default ScoreDetail;
