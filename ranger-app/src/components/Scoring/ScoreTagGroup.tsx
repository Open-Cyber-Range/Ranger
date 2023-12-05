import React, {useCallback, useState} from 'react';
import '@blueprintjs/popover2/lib/css/blueprint-popover2.css';
import {type ExerciseRole} from 'src/models/scenario';
import {type RoleScore} from 'src/models/score';
import ScoreTag from './ScoreTag';

const ScoreTagGroup = ({exerciseId, deploymentId, roles, onScoresChange}:
{exerciseId: string;
  deploymentId: string;
  roles: ExerciseRole[];
  onScoresChange?: (roleScores: RoleScore[]) => void;
}) => {
  const [roleScores, setRoleScores] = useState<RoleScore[]>([]);

  const handleRoleScoreChange = useCallback((role: ExerciseRole, score: number) => {
    setRoleScores(previousScores => {
      const existingScore = previousScores.find(roleScore => roleScore.role === role);

      if (existingScore) {
        return previousScores.map(roleScore =>
          roleScore.role === role ? {...roleScore, score} : roleScore,
        );
      }

      return [...previousScores, {role, score}];
    });

    onScoresChange?.(roleScores);
  }, [onScoresChange, roleScores]);

  if (roles) {
    return (
      <div className='flex m-1 mt-auto mb-auto'>
        {roles.map((role: ExerciseRole) => (
          <div key={role} className='flex mr-1'>
            <ScoreTag
              key={role}
              exerciseId={exerciseId}
              deploymentId={deploymentId}
              role={role}
              onTagScoreChange={score => {
                handleRoleScoreChange(role, score);
              }}
            />
          </div>
        ),
        )}
      </div>
    );
  }

  return null;
};

export default ScoreTagGroup;
