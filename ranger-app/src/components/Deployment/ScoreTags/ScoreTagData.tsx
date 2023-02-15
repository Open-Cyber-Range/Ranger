
import React from 'react';
import {Tag, Colors} from '@blueprintjs/core';
import styled from 'styled-components';
import '@blueprintjs/popover2/lib/css/blueprint-popover2.css';
import {ExerciseRole} from 'src/models/entity';
import {useGetScoreByRoleQuery} from 'src/slices/apiSlice';
import {useTranslation} from 'react-i18next';

const TagWrapper = styled.div`
  display: flex;
  margin-right: 0.5rem; 
`;

const getRoleColor = (role: ExerciseRole) => {
  switch (role) {
    case (ExerciseRole.Red):
      return Colors.RED2;
    case (ExerciseRole.Green):
      return Colors.GREEN3;
    case (ExerciseRole.Blue):
      return Colors.BLUE2;
    case (ExerciseRole.White):
      return Colors.GRAY4;
    default:
      return Colors.GRAY1;
  }
};

const Tagger = ({role, score}: {role: ExerciseRole; score: number}) => {
  const {t} = useTranslation();

  return (
    <Tag
      round
      style={{background: getRoleColor(role)}}
    >
      {role} {t('common.team')}: {score}
    </Tag>
  );
};

const ScoreTagData = ({exerciseId, deploymentId, role}:
{exerciseId: string;
  deploymentId: string;
  role: ExerciseRole;
}) => {
  const {data: score} = useGetScoreByRoleQuery({
    exerciseId,
    deploymentId,
    role,
  });

  const tagScore = Math.round((score ?? 0) * 100) / 100;
  return (
    <TagWrapper key={role}>
      <Tagger
        key={role}
        role={role}
        score={tagScore}
      />
    </TagWrapper>
  );
};

export default ScoreTagData;
