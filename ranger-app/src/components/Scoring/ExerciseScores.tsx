import type React from 'react';
import PageHolder from 'src/components/PageHolder';
import {useTranslation} from 'react-i18next';
import type {Deployment} from 'src/models/deployment';
import {Callout, H4, HTMLSelect} from '@blueprintjs/core';
import {useNavigate} from 'react-router-dom';
import ScoreTagGroup from 'src/components/Scoring/ScoreTagGroup';
import {useCallback, useEffect, useState} from 'react';
import useFetchRolesForDeployment from 'src/hooks/useFetchRolesForDeployment';
import {type ExerciseRole} from 'src/models/scenario';
import useGetAllRoles from 'src/hooks/useGetAllRoles';
import {toastWarning} from 'src/components/Toaster';
import {getExerciseRoleFromString, sortDeployments} from 'src/utils/score';
import {type DeploymentScore, type RoleScore} from 'src/models/score';

const ScoresPanel = ({deployments}:
{
  deployments: Deployment[] | undefined;
}) => {
  const {t} = useTranslation();
  const navigate = useNavigate();
  const {fetchedRoles, fetchRolesForDeployment} = useFetchRolesForDeployment();
  const {roles, isError} = useGetAllRoles(deployments, fetchedRoles, fetchRolesForDeployment);
  const [selectedRole, setSelectedRole] = useState<string>('all');
  const [deploymentRoles, setDeploymentRoles] = useState<ExerciseRole[]>(roles);
  const [deploymentScores, setDeploymentScores] = useState<DeploymentScore[]>([]);
  const [sortedDeployments, setSortedDeployments] = useState<Deployment[]>([]);
  const [sortOrder, setSortOrder] = useState<string>('scoreDesc');

  const handleScoresChange = useCallback((deploymentId: string, roleScores: RoleScore[]) => {
    setDeploymentScores(previousScores => {
      const existingScore = previousScores.find(score => score.deploymentId === deploymentId);

      if (existingScore) {
        return previousScores.map(score =>
          score.deploymentId === deploymentId ? {...score, roleScores} : score,
        );
      }

      return [...previousScores, {deploymentId, roleScores}];
    });
  }, []);

  const handleClick = (deploymentId: string) => {
    navigate(`deployments/${deploymentId}`);
  };

  const handleRoleChange = useCallback((event: React.ChangeEvent<HTMLSelectElement>) => {
    setSelectedRole(event.target.value);
  }, []);

  const handleSortOrderChange = useCallback((event: React.ChangeEvent<HTMLSelectElement>) => {
    setSortOrder(event.target.value);
  }, []);

  useEffect(() => {
    if (selectedRole === 'all') {
      setDeploymentRoles(roles);
    } else {
      const selectedExerciseRole = getExerciseRoleFromString(selectedRole);
      setDeploymentRoles(roles.filter(role => role === selectedExerciseRole));
    }
  }
  , [selectedRole, roles]);

  useEffect(() => {
    if (deployments) {
      setSortedDeployments(sortDeployments(selectedRole, deployments, deploymentScores, sortOrder));
    }
  }
  , [selectedRole, deployments, deploymentScores, sortOrder]);

  useEffect(() => {
    if (isError) {
      toastWarning(t('scoreTable.errorFetchingRoles'));
    }
  }
  , [isError, t]);

  if (deployments) {
    return (
      <PageHolder>
        <div className='flex flex-row justify-between  mb-2'>
          <HTMLSelect
            value={selectedRole}
            onChange={handleRoleChange}
          >
            <option value='all'>{t('scoreTable.allRoles')}</option>
            {roles.map((role: ExerciseRole) => (
              <option key={role} value={role}>{role}</option>
            ))}
          </HTMLSelect>

          <HTMLSelect
            value={sortOrder}
            onChange={handleSortOrderChange}
          >
            <option value='scoreDesc'>{t('scoreTable.scoreDescending')}</option>
            <option value='scoreAsc'>{t('scoreTable.scoreAscending')}</option>
            <option value='updateDesc'>{t('scoreTable.updatedAtDescending')}</option>
            <option value='updateAsc'>{t('scoreTable.updatedAtAscending')}</option>
          </HTMLSelect>
        </div>
        <div className='flex flex-col'>
          <table className='
              bp4-html-table
              bp4-html-table-striped
              bp4-interactive'
          >
            <tbody>
              {sortedDeployments.map(deployment => (
                <tr
                  key={deployment.id}
                  onClick={() => {
                    handleClick(deployment.id);
                  }}
                >
                  <td className='flex flex-row justify-between'>
                    <H4 className='mb-0'>{deployment.name}</H4>
                    <ScoreTagGroup
                      exerciseId={deployment.exerciseId}
                      deploymentId={deployment.id}
                      roles={deploymentRoles}
                      onScoresChange={(roleScores: RoleScore[]) => {
                        handleScoresChange(deployment.id, roleScores);
                      }}
                    />
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </PageHolder>
    );
  }

  return (
    <Callout title={t('exercises.noDeployments') ?? ''}/>
  );
};

export default ScoresPanel;
