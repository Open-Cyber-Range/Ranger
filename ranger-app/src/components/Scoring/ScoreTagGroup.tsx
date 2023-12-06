import React, {useCallback, useEffect, useRef, useState} from 'react';
import '@blueprintjs/popover2/lib/css/blueprint-popover2.css';
import {type ExerciseRole} from 'src/models/scenario';
import {type RoleScore} from 'src/models/score';
import {useAdminGetDeploymentScenarioQuery} from 'src/slices/apiSlice';
import {getExerciseRoleFromString, getRolesFromScenario} from 'src/utils/score';
import ScoreTag from './ScoreTag';

const ScoreTagGroup = ({exerciseId, deploymentId, selectedRole, onRolesChange, onScoresChange}:
{exerciseId: string;
  deploymentId: string;
  selectedRole: string;
  onRolesChange?: (roles: ExerciseRole[]) => void;
  onScoresChange?: (roleScores: RoleScore[]) => void;
}) => {
  const queryParameters = {exerciseId, deploymentId};
  const {data: scenario} = useAdminGetDeploymentScenarioQuery(queryParameters);
  const [roleScores, setRoleScores] = useState<RoleScore[]>([]);
  const [roles, setRoles] = useState<ExerciseRole[]>([]);
  const [sortedRoles, setSortedRoles] = useState<ExerciseRole[]>(roles);
  const previousRoleScoresRef = useRef<RoleScore[]>([]);

  useEffect(() => {
    if (scenario) {
      const scenarioRoles = getRolesFromScenario(scenario);
      onRolesChange?.(scenarioRoles);
      setRoles(scenarioRoles);
    }
  }
  , [scenario, onRolesChange]);

  useEffect(() => {
    if (roles && roles.length > 0) {
      if (selectedRole === 'all' || selectedRole === '') {
        setSortedRoles(roles);
      } else {
        const selectedExerciseRole = getExerciseRoleFromString(selectedRole);
        setSortedRoles(roles.filter(role => role === selectedExerciseRole));
      }
    }
  }
  , [selectedRole, roles]);

  useEffect(() => {
    if (previousRoleScoresRef.current !== roleScores) {
      onScoresChange?.(roleScores);
      previousRoleScoresRef.current = roleScores;
    }
  }, [roleScores, onScoresChange]);

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
  }, []);

  if (sortedRoles) {
    return (
      <div className='flex m-1 mt-auto mb-auto'>
        {sortedRoles.map((role: ExerciseRole) => (
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
