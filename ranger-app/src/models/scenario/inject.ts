import {type Source} from './common';

export type Inject = {
  source?: Source;
  from_entity?: string;
  to_entities?: string[];
  tlos?: string[];
  capabilities?: string[];
};

export type Injects = Record<string, Inject>;
