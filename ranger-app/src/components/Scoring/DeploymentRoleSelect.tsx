import type React from 'react';
import {HTMLSelect} from '@blueprintjs/core';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {useTranslation} from 'react-i18next';
import {sortByProperty} from 'sort-by-property';
import {type Deployment} from 'src/models/deployment';
import {type ExerciseRole} from 'src/models/scenario';
import {useAdminGetDeploymentScenarioQuery} from 'src/slices/apiSlice';
import {getRolesFromScenario} from 'src/utils/score';

const DeploymentRoleSelect = ({deployments, selectedRole, handleRoleChange}:
{deployments: Deployment[];
  selectedRole: string;
  handleRoleChange: (event: React.ChangeEvent<HTMLSelectElement>) => void;
}) => {
  const {t} = useTranslation();
  const deploymentsByCreatedAt = deployments.slice().sort(sortByProperty('createdAt', 'asc'));
  const lastDeployment = deploymentsByCreatedAt[deployments.length - 1];
  const exerciseId = lastDeployment.exerciseId;
  const deploymentId = lastDeployment.id;
  const queryArguments = exerciseId && deploymentId ? {exerciseId, deploymentId} : skipToken;
  const {data: scenario} = useAdminGetDeploymentScenarioQuery(queryArguments);

  if (!scenario) {
    return null;
  }

  const roles = getRolesFromScenario(scenario);

  return (
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
  );
};

export default DeploymentRoleSelect;
