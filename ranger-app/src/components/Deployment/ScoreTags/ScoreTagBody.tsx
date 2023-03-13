/* eslint-disable unicorn/no-array-reduce */
import React from 'react';
import {Tag} from '@blueprintjs/core';
import styled from 'styled-components';
import '@blueprintjs/popover2/lib/css/blueprint-popover2.css';
import {
  useGetDeploymentSchemaQuery,
  useGetDeploymentScoresQuery,
} from 'src/slices/apiSlice';
import {useTranslation} from 'react-i18next';
import {
  findLatestScoreElement,
  getRoleColor,
  isNonNullable,
  roundToDecimalPlaces,
} from 'src/utils';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {type ExerciseRole} from 'src/models/scenario/entity';
import {type Schema} from 'src/models/schema';
import {type ScoreElement} from 'src/models/scoreElement';

const TagWrapper = styled.div`
  display: flex;
  margin-right: 0.5rem;
`;

const calculateTotalScoreForRole = ({schema, scoreElements, role}: {
  schema: Schema;
  scoreElements: ScoreElement[];
  role: ExerciseRole;
}) => {
  if (scoreElements.length > 0) {
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
    const roleScores = scoreElements.filter(score =>
      roleMetricNames.has(score.metricName));

    const uniqueVmNames = [...new Set(roleScores
      .map(score => score.vmName))];
    const totalRoleScore = uniqueVmNames.reduce((scoreSum, vmName) => {
      const vmScores: ScoreElement[] = roleScores
        .filter(scoreElement => scoreElement.vmName === vmName);
      const latest_score = findLatestScoreElement(vmScores);

      return Number(scoreSum) + Number(latest_score?.value);
    }, 0);
    return totalRoleScore;
  }

  return 0;
};

const ScoreTagBody = ({exerciseId, deploymentId, role}:
{exerciseId: string;
  deploymentId: string;
  role: ExerciseRole;
}) => {
  const queryArguments = exerciseId && deploymentId
    ? {exerciseId, deploymentId} : skipToken;
  const {data: scoreElements} = useGetDeploymentScoresQuery(queryArguments);
  const {data: schema} = useGetDeploymentSchemaQuery(queryArguments);
  const {t} = useTranslation();
  const backgroundColor = getRoleColor(role);

  if (schema && scoreElements && role) {
    const tagScore = calculateTotalScoreForRole({schema, scoreElements, role});

    return (
      <TagWrapper key={role}>
        <Tag
          round
          style={{background: backgroundColor}}
        >
          {role} {t('common.team')}: {roundToDecimalPlaces(tagScore) }
        </Tag>
      </TagWrapper>
    );
  }
};

export default ScoreTagBody;
