import React from 'react';
import '@blueprintjs/popover2/lib/css/blueprint-popover2.css';
import {type ExerciseRole} from 'src/models/scenario';
import ScoreTag from './ScoreTag';

const ScoreTagGroup = ({exerciseId, deploymentId, roles}:
{exerciseId: string;
  deploymentId: string;
  roles: ExerciseRole[];
}) => {
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
