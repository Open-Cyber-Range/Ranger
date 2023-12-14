import React, {useEffect, useState} from 'react';
import {Tag} from '@blueprintjs/core';
import '@blueprintjs/popover2/lib/css/blueprint-popover2.css';
import {useAdminGetDeploymentScoresQuery} from 'src/slices/apiSlice';
import {useTranslation} from 'react-i18next';
import {
  calculateTotalScoreForRole,
  getRoleColor,
  roundToDecimalPlaces,
} from 'src/utils';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {type Scenario, type ExerciseRole} from 'src/models/scenario';

const ScoreTag = ({exerciseId, deploymentId, scenario, role, large = false, onTagScoreChange}:
{exerciseId: string;
  deploymentId: string;
  scenario: Scenario | undefined;
  role: ExerciseRole;
  large?: boolean;
  onTagScoreChange?: (tagScore: number) => void;
}) => {
  const queryArguments = exerciseId && deploymentId ? {exerciseId, deploymentId} : skipToken;
  const {data: scores} = useAdminGetDeploymentScoresQuery(queryArguments);
  const {t} = useTranslation();
  const backgroundColor = getRoleColor(role);
  const [tagScore, setTagScore] = useState<number>(0);

  useEffect(() => {
    if (scenario && scores) {
      setTagScore(roundToDecimalPlaces(calculateTotalScoreForRole({scenario, scores, role})));
      onTagScoreChange?.(tagScore);
    }
  }
  , [scenario, scores, role, onTagScoreChange, tagScore]);

  if (scenario && scores) {
    return (
      <Tag
        key={role}
        round
        large={large}
        style={{background: backgroundColor}}
      >
        {role} {t('common.team')}: {tagScore}
      </Tag>
    );
  }

  return null;
};

export default ScoreTag;
