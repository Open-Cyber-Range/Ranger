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
  if (!a.updatedAt || a.updatedAt < b.updatedAt) {
    return 1;
  }

  if (!b.updatedAt || a.updatedAt > b.updatedAt) {
    return -1;
  }

  return 0;
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
  if (!a.createdAt || a.createdAt < b.createdAt) {
    return 1;
  }

  if (!b.createdAt || a.createdAt > b.createdAt) {
    return -1;
  }

  return 0;
}

export function sortByVmNameAscending<
  T extends ScoreElement >(a: T, b: T) {
  if (!a.vmName || a.vmName < b.vmName) {
    return -1;
  }

  if (!b.vmName || a.vmName > b.vmName) {
    return 1;
  }

  return 0;
}

export function sortByVmNameDescending<
  T extends ScoreElement >(a: T, b: T) {
  return sortByVmNameAscending(a, b) * -1;
}

export const getWebsocketBase = () => {
  const protocol = window.location.protocol === 'https:' ? 'wss' : 'ws';
  const host = window.location.host;
  return `${protocol}://${host}`;
};

export const isDevelopment = () =>
  import.meta.env.DEV;
