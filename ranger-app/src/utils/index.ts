import {Colors} from '@blueprintjs/core';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import type {Deployment} from 'src/models/deployment';
import {ExerciseRole} from 'src/models/entity';
import type {Exercise} from 'src/models/exercise';
import type {ScoreElement} from 'src/models/tlo';

export function sortByUpdatedAtAscending<
  T extends Deployment | Exercise>(a: T, b: T) {
  if (!a.updatedAt || a.updatedAt < b.updatedAt) {
    return -1;
  }

  if (!b.updatedAt || a.updatedAt > b.updatedAt) {
    return 1;
  }

  return 0;
}

export function sortByUpdatedAtDescending<
  T extends Deployment | Exercise>(a: T, b: T) {
  return sortByUpdatedAtAscending(a, b) * -1;
}

export function sortByCreatedAtAscending<
  T extends Deployment | Exercise | ScoreElement >(a: T, b: T) {
  if (!a.createdAt || a.createdAt < b.createdAt) {
    return -1;
  }

  if (!b.createdAt || a.createdAt > b.createdAt) {
    return 1;
  }

  return 0;
}

export function sortByCreatedAtDescending<
  T extends Deployment | Exercise | ScoreElement >(a: T, b: T) {
  return sortByCreatedAtAscending(a, b);
}

export const getWebsocketBase = () => {
  const protocol = window.location.protocol === 'https:' ? 'wss' : 'ws';
  const host = window.location.host;
  return `${protocol}://${host}`;
};

export const isDevelopment = () =>
  import.meta.env.DEV;

export const definedOrSkipToken = (
  exerciseId: string | undefined,
  deploymentId: string | undefined) => {
  if (!exerciseId || !deploymentId) {
    return skipToken;
  }

  return {exerciseId, deploymentId};
};

export function groupByMetricNameAndVmName(array: ScoreElement[]) {
  // eslint-disable-next-line unicorn/no-array-reduce
  return array.reduce<Record<string, ScoreElement[]>>(
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
