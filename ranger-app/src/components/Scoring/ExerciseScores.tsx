import type React from 'react';
import PageHolder from 'src/components/PageHolder';
import {useTranslation} from 'react-i18next';
import type {Deployment} from 'src/models/deployment';
import {Callout, H4, HTMLSelect} from '@blueprintjs/core';
import {useNavigate} from 'react-router-dom';
import ScoreTagGroup from 'src/components/Scoring/ScoreTagGroup';
import {useCallback, useEffect, useState} from 'react';
import {type ExerciseRole} from 'src/models/scenario';
import {sortDeployments} from 'src/utils/score';
import {type DeploymentScore, type RoleScore} from 'src/models/score';

const ScoresPanel = ({deployments}:
{
  deployments: Deployment[] | undefined;
}) => {
  const {t} = useTranslation();
  const navigate = useNavigate();
  const [roles, setRoles] = useState<ExerciseRole[]>([]);
  const [selectedRole, setSelectedRole] = useState<string>('');
  const [deploymentScores, setDeploymentScores] = useState<DeploymentScore[]>([]);
  const [sortedDeployments, setSortedDeployments] = useState<Deployment[]>([]);
  const [sortOrder, setSortOrder] = useState<string>('');

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
    if (deployments) {
      setSortedDeployments(sortDeployments(selectedRole, deployments, deploymentScores, sortOrder));
    }
  }
  , [selectedRole, deployments, deploymentScores, sortOrder]);

  if (deployments) {
    return (
      <PageHolder>
        <div className='flex justify-end space-x-2 mb-2'>
          <HTMLSelect
            value={selectedRole}
            onChange={handleRoleChange}
          >
            <option value=''>{t('scoreTable.rolePlaceholder')}</option>
            <option value='all'>{t('scoreTable.allRoles')}</option>
            {roles.map((role: ExerciseRole) => (
              <option key={role} value={role}>{role}</option>
            ))}
          </HTMLSelect>

          <HTMLSelect
            value={sortOrder}
            onChange={handleSortOrderChange}
          >
            <option value=''>{t('scoreTable.orderPlaceholder')}</option>
            <option value='scoreDesc'>{t('scoreTable.scoreDescending')}</option>
            <option value='scoreAsc'>{t('scoreTable.scoreAscending')}</option>
            <option value='nameDesc'>{t('scoreTable.nameDescending')}</option>
            <option value='nameAsc'>{t('scoreTable.nameAscending')}</option>
            <option value='createdDesc'>{t('scoreTable.createdAtDescending')}</option>
            <option value='createdAsc'>{t('scoreTable.createdAtAscending')}</option>
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
                      selectedRole={selectedRole}
                      onRolesChange={(roles: ExerciseRole[]) => {
                        setRoles(roles);
                      }}
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
