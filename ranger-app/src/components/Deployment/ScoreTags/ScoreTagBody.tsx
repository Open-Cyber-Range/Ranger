/* eslint-disable unicorn/no-array-reduce */
import React from 'react';
import {Tag} from '@blueprintjs/core';
import '@blueprintjs/popover2/lib/css/blueprint-popover2.css';
import {
  useGetDeploymentSchemaQuery,
  useGetDeploymentScoresQuery,
} from 'src/slices/apiSlice';
import {useTranslation} from 'react-i18next';
import {
  findLatestScore,
  getRoleColor,
  groupBy,
  isNonNullable,
  roundToDecimalPlaces,
} from 'src/utils';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {type ExerciseRole} from 'src/models/scenario/entity';
import {type Schema} from 'src/models/schema';
import {type Score} from 'src/models/score';

const calculateTotalScoreForRole = ({schema, scores, role}: {
  schema: Schema;
  scores: Score[];
  role: ExerciseRole;
}) => {
  if (scores.length > 0) {
    const entityValues = Object.values(schema.entities);
    const roleEntities = entityValues.slice().filter(entity =>
      entity.role?.valueOf() === role,
    );

    const roleTloNames = roleEntities.flatMap(entity =>
      entity.goals?.flatMap(goal_ref => schema.goals[goal_ref]?.tlos));
    // eslint-disable-next-line unicorn/no-array-callback-reference
    const roleEvaluationNames = roleTloNames.filter(isNonNullable)
      .flatMap(tloName => schema.tlos[tloName]?.evaluation);
    const roleMetricNames = new Set(roleEvaluationNames
      .flatMap(evaluationName =>
        schema.evaluations[evaluationName]?.metrics));
    const roleScores = scores.filter(score =>
      roleMetricNames.has(score.metricName));

    const uniqueVmNames = [...new Set(roleScores.map(score => score.vmName))];
    const totalRoleScore = uniqueVmNames.reduce((scoreSum, vmName) => {
      const vmScores: Score[] = roleScores.filter(score =>
        score.vmName === vmName);

      const scoresByMetric = groupBy(vmScores, score => score.metricName);

      for (const metricName in scoresByMetric) {
        if (scoresByMetric[metricName]) {
          const currentScore = findLatestScore(scoresByMetric[metricName]);
          if (currentScore?.value) {
            scoreSum += Number(currentScore?.value);
          }
        }
      }

      return scoreSum;
    }, 0);
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
  const {data: schema} = useGetDeploymentSchemaQuery(queryArguments);
  const {t} = useTranslation();
  const backgroundColor = getRoleColor(role);

  if (schema && scores && role) {
    const tagScore = calculateTotalScoreForRole({schema, scores, role});

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
