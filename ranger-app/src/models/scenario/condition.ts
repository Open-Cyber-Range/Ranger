import {type Source} from './common';

export type Condition = {
  command?: string;
  interval?: number;
  source?: Source;
};

export type Conditions = Record<string, Condition>;
