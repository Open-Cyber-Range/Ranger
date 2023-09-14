import React from 'react';
import {Tag} from '@blueprintjs/core';
import '@blueprintjs/popover2/lib/css/blueprint-popover2.css';
import {
  useAdminGetDeploymentScenarioQuery,
  useAdminGetDeploymentScoresQuery,
} from 'src/slices/apiSlice';
import {useTranslation} from 'react-i18next';
import {
  calculateTotalScoreForRole,
  getRoleColor,
  roundToDecimalPlaces,
} from 'src/utils';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {type ExerciseRole} from 'src/models/scenario';

const ScoreTag = ({exerciseId, deploymentId, role, large = false}:
{exerciseId: string;
  deploymentId: string;
  role: ExerciseRole;
  large?: boolean;
}) => {
  const queryArguments = exerciseId && deploymentId
    ? {exerciseId, deploymentId} : skipToken;
  const {data: scores} = useAdminGetDeploymentScoresQuery(queryArguments);
  const {data: scenario} = useAdminGetDeploymentScenarioQuery(queryArguments);
  const {t} = useTranslation();
  const backgroundColor = getRoleColor(role);

  if (scenario && scores) {
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

export default ScoreTag;
