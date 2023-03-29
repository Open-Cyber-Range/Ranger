import {Colors} from '@blueprintjs/core';
import {ExerciseRole} from 'src/models/scenario';
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
