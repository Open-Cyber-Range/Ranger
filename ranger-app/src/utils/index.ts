import {Colors} from '@blueprintjs/core';
import {
  type Entity,
  ExerciseRole,
  type Goal,
  type TloMapsByRole,
  type TrainingLearningObjective,
} from 'src/models/scenario';
import {type Score} from 'src/models/score';

export const getWebsocketBase = () => {
  const protocol = window.location.protocol === 'https:' ? 'wss' : 'ws';
  const host = window.location.host;
  return `${protocol}://${host}`;
};

export const isDevelopment = () =>
  import.meta.env.DEV;

export function groupByMetricNameAndVmName(array: Score[]) {
  return array.reduce<Record<string, Score[]>>(
    (groupedMap, element) => {
      const key = element.metricName.concat(' - ', element.vmName);
      if (groupedMap[key]) {
        groupedMap[key].push(element);
      } else {
        groupedMap[key] = [element];
      }

      return groupedMap;
    }, {});
}

export const groupBy = <T, K extends keyof any>(array: T[], key: (i: T) => K) =>
  array.reduce<Record<K, T[]>>((groups, item) => {
    (groups[key(item)] ||= []).push(item);
    return groups;
  // eslint-disable-next-line @typescript-eslint/prefer-reduce-type-parameter
  }, {} as Record<K, T[]>);

export const defaultColors = [
  '#147EB3',
  '#29A634',
  '#D1980B',
  '#D33D17',
  '#9D3F9D',
  '#00A396',
  '#DB2C6F',
  '#8EB125',
  '#946638',
  '#7961DB',
];

export const getRoleColor = (role: ExerciseRole) => {
  switch (role) {
    case (ExerciseRole.Red): {
      return Colors.RED2;
    }

    case (ExerciseRole.Green): {
      return Colors.GREEN3;
    }

    case (ExerciseRole.Blue): {
      return Colors.BLUE2;
    }

    case (ExerciseRole.White): {
      return Colors.GRAY4;
    }

    default: {
      return Colors.GRAY1;
    }
  }
};

export const roundToDecimalPlaces
= (value: number, decimalPlaces = 2): number => {
  const scale = 10 ** decimalPlaces;
  return Math.round(value * scale) / scale;
};

export type RequireAtLeastOne<T, Keys extends keyof T = keyof T> =
    Pick<T, Exclude<keyof T, Keys>>
    & {
      [K in Keys]-?: Required<Pick<T, K>> & Partial<Pick<T, Exclude<Keys, K>>>
    }[Keys];

export function isNonNullable<T>(value: T): value is NonNullable<T> {
  return value !== null && value !== undefined;
}

export const findLatestScore = (scores: Score[]) => {
  if (scores.length > 0) {
    const latestScore = scores.reduce((previous, current) =>
      (Date.parse(previous?.timestamp)
      > Date.parse(current?.timestamp)) ? previous : current);
    return latestScore;
  }

  return undefined;
};

export const findLatestScoresByVms = (scores: Score[]) => {
  const uniqueVmNames = [...new Set(scores
    .map(score => score.vmName))];
  const latestScoresByVm = uniqueVmNames
    .reduce<Score[]>((latestVms, vmName) => {
    const scoresByVm = scores.filter(score =>
      score.vmName === vmName);
    const latestScoreValue = findLatestScore(scoresByVm);
    if (latestScoreValue) {
      latestVms.push(latestScoreValue);
      return latestVms;
    }

    return [];
  }, []);

  return latestScoresByVm;
};

export function sumScoresByMetric(
  metricNames: string[], scoresByMetric: Record<string, Score[]>) {
  return metricNames.reduce(
    (metricScoreSum, metricName) => {
      if (scoresByMetric[metricName]) {
        const currentScore = findLatestScore(scoresByMetric[metricName]);
        if (currentScore?.value) {
          metricScoreSum += Number(currentScore?.value);
        }
      }

      return metricScoreSum;
    }, 0);
}

export function sumScoresByRole(uniqueVmNames: string[], roleScores: Score[]) {
  return uniqueVmNames.reduce((totalScoreSum, vmName) => {
    const vmScores = roleScores.filter(score => score.vmName === vmName);

    const scoresByMetric = groupBy(vmScores, score => score.metricName);
    const metricNames = Object.keys(scoresByMetric);
    const metricScoreSum = sumScoresByMetric(metricNames, scoresByMetric);
    totalScoreSum += metricScoreSum;
    return totalScoreSum;
  }, 0);
}

export function getTloNamesByRole(
  entities: Record<string, Entity>,
  goals: Record<string, Goal>,
  role: ExerciseRole) {
  const entityValues = Object.values(entities);
  const roleEntities = entityValues.slice().filter(entity =>
    entity.role?.valueOf() === role,
  );

  const tloNames = roleEntities.slice().reduce<string []>(
    (tloNames, entity) => {
      if (entity.goals) {
        for (const goalName of entity.goals) {
          tloNames = tloNames.concat(goals[goalName]?.tlos);
        }
      }

      return tloNames;
    }, []);
  return tloNames;
}

export function groupTloMapsByRoles(
  entities: Record<string, Entity>,
  goals: Record<string, Goal>,
  tlos: Record<string, TrainingLearningObjective>,
  roles: ExerciseRole[],
) {
  const tloMapsByRole = roles.reduce<TloMapsByRole>((tloMapsByRole, role) => {
    const roleTloNames = getTloNamesByRole(entities, goals, role);
    const roleTloMap
      = roleTloNames.reduce<Record<string, TrainingLearningObjective>>(
        (roleTloMap, tloName) => {
          if (tlos[tloName]) {
            roleTloMap[tloName] = tlos[tloName];
          }

          return roleTloMap;
        }, {});

    tloMapsByRole[role] = roleTloMap;
    return tloMapsByRole;
  }, {});
  return tloMapsByRole;
}
