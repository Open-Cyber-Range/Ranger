import type {Deployment} from 'src/models/deployment';
import type {Exercise} from 'src/models/exercise';

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
