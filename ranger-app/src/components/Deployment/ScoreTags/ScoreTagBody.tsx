import React from 'react';
import {Tag} from '@blueprintjs/core';
import styled from 'styled-components';
import '@blueprintjs/popover2/lib/css/blueprint-popover2.css';
import type {ExerciseRole} from 'src/models/entity';
import {useGetScoreByRoleQuery} from 'src/slices/apiSlice';
import {useTranslation} from 'react-i18next';
import {getRoleColor, roundToDecimalPlaces} from 'src/utils';

const TagWrapper = styled.div`
  display: flex;
  margin-right: 0.5rem; 
`;

const ScoreTagBody = ({exerciseId, deploymentId, role}:
{exerciseId: string;
  deploymentId: string;
  role: ExerciseRole;
}) => {
  const {data: score} = useGetScoreByRoleQuery({
    exerciseId,
    deploymentId,
    role,
  });

  const {t} = useTranslation();
  const backgroundColor = getRoleColor(role);
  const tagScore = roundToDecimalPlaces(score ?? 0);

  return (
    <TagWrapper key={role}>
      <Tag
        round
        style={{background: backgroundColor}}
      >
        {role} {t('common.team')}: {tagScore}
      </Tag>
    </TagWrapper>
  );
};

export default ScoreTagBody;
