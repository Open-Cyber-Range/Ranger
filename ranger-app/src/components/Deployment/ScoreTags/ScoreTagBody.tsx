import React from 'react';
import {Tag} from '@blueprintjs/core';
import '@blueprintjs/popover2/lib/css/blueprint-popover2.css';
import {
  useGetDeploymentScenarioQuery,
  useGetDeploymentScoresQuery,
} from 'src/slices/apiSlice';
import {useTranslation} from 'react-i18next';
import {
  getRoleColor,
  getTloNamesByRole,
  roundToDecimalPlaces,
  sumScoresByRole,
} from 'src/utils';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {type Score} from 'src/models/score';
import {type Scenario, type ExerciseRole} from 'src/models/scenario';

const calculateTotalScoreForRole = ({scenario, scores, role}: {
  scenario: Scenario;
  scores: Score[];
  role: ExerciseRole;
}) => {
  const {entities, goals, tlos, evaluations} = scenario;

  if (entities && goals && tlos && evaluations && scores.length > 0) {
    const roleTloNames = getTloNamesByRole(entities, goals, role);
    const roleEvaluationNames = roleTloNames.flatMap(tloName =>
      tlos[tloName]?.evaluation);
    const roleMetricNames = new Set(roleEvaluationNames
      .flatMap(evaluationName =>
        evaluations[evaluationName]?.metrics));
    const roleScores = scores.filter(score =>
      roleMetricNames.has(score.metricName));

    const uniqueVmNames = [...new Set(roleScores.map(score => score.vmName))];
    const totalRoleScore = sumScoresByRole(uniqueVmNames, roleScores);
    return totalRoleScore;
  }

  return 0;
};

const ScoreTagBody = ({exerciseId, deploymentId, role, large = false}:
{exerciseId: string;
  deploymentId: string;
  role: ExerciseRole;
  large?: boolean;
}) => {
  const queryArguments = exerciseId && deploymentId
    ? {exerciseId, deploymentId} : skipToken;
  const {data: scores} = useGetDeploymentScoresQuery(queryArguments);
  const {data: scenario} = useGetDeploymentScenarioQuery(queryArguments);
  const {t} = useTranslation();
  const backgroundColor = getRoleColor(role);

  if (scenario && scores && role) {
    const tagScore = calculateTotalScoreForRole({scenario, scores, role});

    return (
      <Tag
        key={role}
        round
        large={large}
        style={{background: backgroundColor}}
      >
        {role} {t('common.team')}: {roundToDecimalPlaces(tagScore) }
      </Tag>
    );
  }

  return null;
};

export default ScoreTagBody;
