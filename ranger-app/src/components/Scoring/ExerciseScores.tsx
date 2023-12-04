import type React from 'react';
import PageHolder from 'src/components/PageHolder';
import {useTranslation} from 'react-i18next';
import type {Deployment} from 'src/models/deployment';
import {Callout, H4, HTMLSelect} from '@blueprintjs/core';
import {useNavigate} from 'react-router-dom';
import {sortByProperty} from 'sort-by-property';
import ScoreTagGroup from 'src/components/Scoring/ScoreTagGroup';
import {useEffect, useState} from 'react';
import useFetchRolesForDeployment from 'src/hooks/useFetchRolesForDeployment';
import {type ExerciseRole} from 'src/models/scenario';
import useGetAllRoles from 'src/hooks/useGetAllRoles';
import {toastWarning} from 'src/components/Toaster';
import {getExerciseRoleFromString} from 'src/utils/graph';

const ScoresPanel = ({deployments}:
{
  deployments: Deployment[] | undefined;
}) => {
  const {t} = useTranslation();
  const navigate = useNavigate();
  const {fetchedRoles, fetchRolesForDeployment} = useFetchRolesForDeployment();
  const {roles, isError} = useGetAllRoles(deployments, fetchedRoles, fetchRolesForDeployment);
  const [selectedRole, setSelectedRole] = useState('all');
  const [deploymentRoles, setDeploymentRoles] = useState<ExerciseRole[]>(roles);

  const handleClick = (deploymentId: string) => {
    navigate(`deployments/${deploymentId}`);
  };

  const handleRoleChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    setSelectedRole(event.target.value);
  };

  useEffect(() => {
    if (selectedRole === 'all') {
      setDeploymentRoles(roles);
    } else {
      const selectedExerciseRole = getExerciseRoleFromString(selectedRole);
      setDeploymentRoles(roles.filter(role => role === selectedExerciseRole));
    }

    if (isError) {
      toastWarning(t('roles.errorFetchingRoles'));
    }
  }
  , [selectedRole, roles, isError, t]);

  if (deployments) {
    deployments = deployments.slice().sort(sortByProperty('updatedAt', 'desc'));

    return (
      <PageHolder>
        <div className='flex flex-row items-end mb-2'>
          <HTMLSelect
            value={selectedRole}
            onChange={handleRoleChange}
          >
            <option value='all'>{t('roles.allRoles')}</option>
            {roles.map((role: ExerciseRole) => (
              <option key={role} value={role}>{role}</option>
            ))}
          </HTMLSelect>
        </div>
        <div className='flex flex-col'>
          <table className='
              bp4-html-table
              bp4-html-table-striped
              bp4-interactive'
          >
            <tbody>
              {deployments.map(deployment => (
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
                      roles={deploymentRoles}/>
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
