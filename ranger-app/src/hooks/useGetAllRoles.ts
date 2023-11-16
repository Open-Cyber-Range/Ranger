
import {useState, useEffect} from 'react';
import {type Deployment} from 'src/models/deployment';
import {type ExerciseRole} from 'src/models/scenario';

const useGetAllRoles = (
  deployments: Deployment[] | undefined,
  fetchedRoles: ExerciseRole[],
  fetchRolesForDeployment: (exerciseId: string, deploymentId: string) => Promise<void>) => {
  const [roles, setRoles] = useState<ExerciseRole[]>([]);
  const [isFetched, setIsFetched] = useState(false);
  const [isError, setIsError] = useState<boolean>(false);

  useEffect(() => {
    async function fetchRoles() {
      if (!deployments) {
        return;
      }

      try {
        await Promise.all(deployments.map(async deployment =>
          fetchRolesForDeployment(deployment.exerciseId, deployment.id)));
        setRoles(fetchedRoles);
      } catch {
        setIsError(true);
      } finally {
        setIsFetched(true);
      }
    }

    if (deployments && !isFetched) {
      fetchRoles().catch(_ => {
        setIsError(true);
      });
    }
  }, [deployments, fetchedRoles, fetchRolesForDeployment, isFetched]);

  return {roles, isError};
};

export default useGetAllRoles;
