import React, {useState} from 'react';
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

  const handleScoreChange = (role: ExerciseRole, score: number) => {
    setRoleScores(previousScores => {
      const newScores = [...previousScores];
      const index = newScores.findIndex(r => r.role === role);

      if (index > -1) {
        newScores[index] = {role, score};
      } else {
        newScores.push({role, score});
      }

      return newScores;
    });

    onScoresChange?.(roleScores);
  };

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
                handleScoreChange(role, score);
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
