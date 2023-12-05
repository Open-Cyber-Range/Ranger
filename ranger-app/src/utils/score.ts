import {sortByProperty} from 'sort-by-property';
import {type Deployment} from 'src/models/deployment';
import {ExerciseRole} from 'src/models/scenario';
import {type DeploymentScore} from 'src/models/score';

export function getExerciseRoleFromString(role: string): ExerciseRole | undefined {
  const roles = Object.keys(ExerciseRole) as ExerciseRole[];
  return roles.find(r => r.toLowerCase() === role.toLowerCase());
}

function calculateTotalScore(
  deploymentScores: DeploymentScore[],
  deploymentId: string): number {
  const deploymentScore = deploymentScores.find(ds => ds.deploymentId === deploymentId);
  return deploymentScore
    ? deploymentScore.roleScores.reduce((total, rs) => total + rs.score, 0) : 0;
}

export const sortDeployments = (
  selectedRole: string,
  deployments: Deployment[],
  deploymentScores: DeploymentScore[],
  sortOrder: string) => {
  if (sortOrder.includes('update')) {
    const order = sortOrder === 'updateDesc' ? 'desc' : 'asc';
    return deployments.slice().sort(sortByProperty('updatedAt', order));
  }

  if (sortOrder.includes('score')) {
    const isDescending = sortOrder === 'scoreDesc';

    return deployments.slice().sort((a, b) => {
      let scoreA;
      let scoreB;

      if (selectedRole === 'all') {
        scoreA = calculateTotalScore(deploymentScores, a.id);
        scoreB = calculateTotalScore(deploymentScores, b.id);
      } else {
        const deploymentScoreA = deploymentScores.find(ds => ds.deploymentId === a.id);
        const deploymentScoreB = deploymentScores.find(ds => ds.deploymentId === b.id);

        scoreA = deploymentScoreA?.roleScores.find(rs => rs.role === selectedRole)?.score ?? 0;
        scoreB = deploymentScoreB?.roleScores.find(rs => rs.role === selectedRole)?.score ?? 0;
      }

      return isDescending ? scoreB - scoreA : scoreA - scoreB;
    });
  }

  return deployments.slice().sort(sortByProperty('updatedAt', 'desc'));
};

