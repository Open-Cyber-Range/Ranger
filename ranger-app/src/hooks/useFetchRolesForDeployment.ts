import {flattenEntities, getTloKeysByRole, getUniqueRoles} from 'src/utils';
import {
  ExerciseRoleOrder,
  type ExerciseRole,
  type Scenario,
} from 'src/models/scenario';
import {useState} from 'react';
import {useSelector} from 'react-redux';
import {type RootState} from 'src/store';

const useFetchRolesForDeployment = () => {
  const [roles, setRoles] = useState<ExerciseRole[]>([]);
  const token = useSelector((state: RootState) => state.user.token);

  const fetchRolesForDeployment = async (exerciseId: string, deploymentId: string) => {
    if (!token) {
      return;
    }

    const response
    = await fetch(`/api/v1/admin/exercise/${exerciseId}/deployment/${deploymentId}/scenario`, {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });
    const scenario = await response.json() as Scenario;
    const entities = scenario?.entities;

    if (!entities) {
      return;
    }

    const flattenedEntities = flattenEntities(entities);
    const fetchedRoles = getUniqueRoles(flattenedEntities);

    const rolesWithTlos: ExerciseRole[] = [];

    for (const role of fetchedRoles) {
      const roleTloNames = getTloKeysByRole(flattenedEntities, role);
      if (roleTloNames.length > 0) {
        rolesWithTlos.push(role);
      }
    }

    rolesWithTlos.sort((a, b) => ExerciseRoleOrder[a] - ExerciseRoleOrder[b]);
    setRoles(rolesWithTlos);
  };

  return {
    fetchedRoles: roles,
    fetchRolesForDeployment,
  };
};

export default useFetchRolesForDeployment;
