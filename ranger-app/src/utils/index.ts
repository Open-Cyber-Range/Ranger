import {Colors, type TreeNodeInfo} from '@blueprintjs/core';
import {type AdUser} from 'src/models/groups';
import {type Participant} from 'src/models/pariticpant';
import {
  type Entity,
  type Evaluation,
  ExerciseRole,
  type Metric,
  type Scenario,
  type TloMapsByRole,
  type TrainingLearningObjective,
} from 'src/models/scenario';
import {type Score} from 'src/models/score';

export const createEntityTree = (
  entities: Record<string, Entity>,
  participants: Participant[] = [],
  users: AdUser[] = [],
  selector?: string,
): TreeNodeInfo[] => {
  const sortedEntityKeys = Object.keys(entities).sort((a, b) => {
    const entityA = entities[a];
    const entityB = entities[b];
    return (entityA.name ?? a).localeCompare(entityB.name ?? b);
  });

  const tree: TreeNodeInfo[] = [];
  for (const entityId of sortedEntityKeys) {
    const entity = entities[entityId];
    const id = selector ? `${selector}.${entityId}` : entityId;
    const matchingParticipant = participants.find(participant => participant.selector === id);
    const matchingUser = users.find(user => user.id === matchingParticipant?.userId);
    const entityNode: TreeNodeInfo = {
      id,
      label: `${entity.name ?? entityId}${matchingUser ? ': ' : ''}${matchingUser?.username ?? ''}`,
      icon: 'person',
      isExpanded: true,
    };
    if (entity.entities) {
      entityNode.childNodes = createEntityTree(entity.entities, participants, users, id);
    }

    tree.push(entityNode);
  }

  return tree;
};

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

export function getTloKeysByRole(
  entities: Record<string, Entity>,
  role: ExerciseRole) {
  const entityValues = Object.values(entities);
  const roleEntities = entityValues.slice().filter(entity =>
    entity.role?.valueOf() === role,
  );
  const tloKeys = roleEntities.slice().reduce<string []>(
    (tloKeys, entity) => {
      if (entity.tlos) {
        tloKeys = tloKeys.concat(entity?.tlos);
      }

      return tloKeys;
    }, []);
  return tloKeys;
}

export function groupTloMapsByRoles(
  entities: Record<string, Entity>,
  tlos: Record<string, TrainingLearningObjective>,
  roles: ExerciseRole[],
) {
  const tloMapsByRole = roles.reduce<TloMapsByRole>((tloMapsByRole, role) => {
    const roleTloNames = getTloKeysByRole(entities, role);
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

export function flattenEntities(entities: Record<string, Entity>) {
  const outputEntities: Record<string, Entity> = {};

  for (const key of Object.keys(entities)) {
    const childEntity = entities[key];
    outputEntities[key] = childEntity;

    if (childEntity.entities) {
      const nested = flattenEntities(childEntity.entities);
      for (const nestedKey of Object.keys(nested)) {
        outputEntities[`${key}.${nestedKey}`] = nested[nestedKey];
      }
    }
  }

  return outputEntities;
}

export function getUniqueRoles(entities: Record<string, Entity>) {
  const rolesSet = Object.values(entities)
    .reduce<Set<ExerciseRole>>((accumulator, entity) => {
    if (entity?.role) {
      accumulator.add(entity.role);
    }

    return accumulator;
  }, new Set<ExerciseRole>());

  return Array.from(rolesSet);
}

export const getTloKeysForEntityOrClosestParent
= (currentEntityKey: string, flattenedEntities: Record<string, Entity>): string[] => {
  let currentEntity = flattenedEntities[currentEntityKey];

  while (currentEntity && !currentEntity.tlos && currentEntityKey.includes('.')) {
    const lastPeriodIndex = currentEntityKey.lastIndexOf('.');
    currentEntityKey = currentEntityKey.slice(0, Math.max(0, lastPeriodIndex));
    currentEntity = flattenedEntities[currentEntityKey];
  }

  return currentEntity?.tlos ?? [];
};

export const getMetricsByEntityKey
= (entityKey: string, scenario: Scenario): Record<string, Metric> => {
  const entities = scenario?.entities;
  const tlos = scenario?.tlos;
  const evaluations = scenario?.evaluations;
  const metrics = scenario?.metrics;

  if (entities && tlos && evaluations && metrics) {
    const flattenedEntities = flattenEntities(entities);
    const entity = flattenedEntities[entityKey];

    if (entity) {
      const entityTloKeys = getTloKeysForEntityOrClosestParent(entityKey, flattenedEntities);
      const entityMetricKeys = entityTloKeys.map(tloKey => tlos[tloKey])
        .map(tlo => tlo.evaluation)
        .map(evaluationKey => evaluations[evaluationKey])
        .flatMap(evaluation => evaluation.metrics);

      const entityMetrics = entityMetricKeys
        .reduce<Record<string, Metric>>((accumulator, metricKey) => {
        if (metrics[metricKey]) {
          accumulator[metricKey] = metrics[metricKey];
        }

        return accumulator;
      }, {});

      return entityMetrics;
    }
  }

  return {};
};

export const calculateTotalScoreForRole = ({scenario, scores, role}: {
  scenario: Scenario;
  scores: Score[];
  role: ExerciseRole;
}) => {
  const {entities = {}, tlos = {}, evaluations = {}, metrics = {}} = scenario ?? {};
  const flattenedEntities = flattenEntities(entities);
  const roleTloNames = getTloKeysByRole(flattenedEntities, role);
  const roleEvaluationNames = roleTloNames.flatMap(tloName =>
    tlos[tloName]?.evaluation ?? []);
  const roleMetricKeys = Array.from(new Set(roleEvaluationNames
    .flatMap(evaluationName =>
      evaluations[evaluationName]?.metrics ?? [])));
  const roleMetrics = new Set(roleMetricKeys
    .map(metricKey => metrics[metricKey]?.name ?? metricKey)
    .filter(Boolean));

  const roleScores = scores.filter(score =>
    roleMetrics.has(score.metricName));

  const uniqueVmNames = [...new Set(roleScores.map(score => score.vmName))];
  const totalRoleScore = sumScoresByRole(uniqueVmNames, roleScores);
  return totalRoleScore;
};

export const getMetricReferencesByRole = (
  entities: Record<string, Entity>,
  tlos: Record<string, TrainingLearningObjective>,
  evaluations: Record<string, Evaluation>,
  metrics: Record<string, Metric>,
) => {
  const flattenedEntities = flattenEntities(entities);
  const result = Object.values(flattenedEntities)
    .reduce<Record<ExerciseRole, Set<string>>>((acc, entity) => {
    const role = entity.role;
    const entityTlos = entity.tlos;
    if (role && entityTlos) {
      const metricReferences = entityTlos.map(tloKey => tlos[tloKey])
        .map(tlo => tlo.evaluation)
        .map(evaluationKey => evaluations[evaluationKey])
        .flatMap(evaluation => evaluation.metrics)
        .map(metricKey => metrics[metricKey].name ?? metricKey);

      for (const metricReference of metricReferences) {
        acc[role].add(metricReference);
      }
    }

    return acc;
  }, {
    [ExerciseRole.Blue]: new Set(),
    [ExerciseRole.Green]: new Set(),
    [ExerciseRole.Red]: new Set(),
    [ExerciseRole.White]: new Set(),
  });

  return result;
};

export const get_table_bg_color_by_role = (role: ExerciseRole) => {
  switch (role) {
    case ExerciseRole.Blue: {
      return 'bg-blue-300';
    }

    case ExerciseRole.White: {
      return 'bg-gray-300';
    }

    case ExerciseRole.Red: {
      return 'bg-red-300';
    }

    case ExerciseRole.Green: {
      return 'bg-green-300';
    }

    default: {
      return 'bg-white';
    }
  }
};

export const get_table_row_bg_color_by_role = (role: ExerciseRole) => {
  switch (role) {
    case ExerciseRole.Blue: {
      return 'bg-blue-50';
    }

    case ExerciseRole.White: {
      return 'bg-gray-50';
    }

    case ExerciseRole.Red: {
      return 'bg-red-50';
    }

    case ExerciseRole.Green: {
      return 'bg-green-50';
    }

    default: {
      return 'bg-white';
    }
  }
};
