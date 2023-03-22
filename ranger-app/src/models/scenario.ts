import {type RequireAtLeastOne} from 'src/utils';

export type Capability = {
  description?: string;
  condition: string;
  vulnerabilities?: string[];
};

export type Condition = {
  command?: string;
  interval?: number;
  source?: Source;
};

export enum ExerciseRole {
  Blue = 'Blue',
  Green = 'Green',
  Red = 'Red',
  White = 'White',
}

export const ExerciseRoleOrder = {
  [ExerciseRole.Blue]: 1,
  [ExerciseRole.Green]: 2,
  [ExerciseRole.Red]: 3,
  [ExerciseRole.White]: 4,
};

export type Entity = {
  name?: string;
  description?: string;
  role?: ExerciseRole;
  mission?: string;
  categories?: string[];
  vulnerabilities?: string[];
  goals?: string[];
  facts?: Record<string, string>;
  entities?: Record<string, Entity>;
};

export type PotentialMinScore = {
  absolute?: number;
  percentage?: number;
};

type MinScore = RequireAtLeastOne<PotentialMinScore, 'absolute' | 'percentage'>;

export type Evaluation = {
  description?: string;
  metrics: string[];
  min_score?: MinScore;
};

type Event = {
  time?: number;
  conditions?: string[];
  injects: string[];
};

export enum FeatureType {
  Service,
  Configuration,
  Artifact,
}

export type Feature = {
  feature_type: FeatureType;
  source?: Source;
  dependencies?: string[];
  vulnerabilities?: string[];
  variables?: Record<string, string>;
  destination?: string;
};

export type Goal = {
  name?: string;
  description?: string;
  tlos: string[];
};

export type InfraNode = {
  count: number;
  links?: string[];
  dependencies?: string[];
};

export type Inject = {
  source?: Source;
  from_entity?: string;
  to_entities?: string[];
  tlos?: string[];
  capabilities?: string[];
};

export enum MetricType {
  Manual,
  Conditional,
}

export type Metric = {
  metric_type: MetricType;
  artifact?: boolean;
  max_score: number;
  condition?: string;
};

export enum NodeType {
  VM,
  Switch,
}

export type Resources = {
  ram: number;
  cpu: number;
};

export type Node = {
  type_field: NodeType;
  description?: string;
  resources?: Resources;
  source?: Source;
  features?: Record<string, string>;
  conditions?: Record<string, string>;
  vulnerabilities?: string[];
  roles?: Record<string, string>;
};

export type Script = {
  start_time: bigint;
  end_time: bigint;
  speed: number;
  events: string[];
};

export type Source = {
  name: string;
  version: string;
};

export type Story = {
  clock: bigint;
  scripts: string[];
};

export type TrainingLearningObjective = {
  name?: string;
  description?: string;
  evaluation: string;
  capabilities?: string[];
};

export type Vulnerability = {
  name: string;
  description: string;
  technical: boolean;
  class: string;
};

export type Scenario = {
  name: string;
  description?: string;
  start: string;
  end: string;
  nodes?: Record<string, Node>;
  features?: Record<string, Feature>;
  infrastructure?: Record<string, InfraNode>;
  conditions?: Record<string, Condition>;
  vulnerabilities?: Record<string, Vulnerability>;
  capabilities?: Record<string, Capability>;
  metrics?: Record<string, Metric>;
  evaluations?: Record<string, Evaluation>;
  tlos?: Record<string, TrainingLearningObjective>;
  entities?: Record<string, Entity>;
  goals?: Record<string, Goal>;
  injects?: Record<string, Inject>;
  events?: Record<string, Event>;
  scripts?: Record<string, Script>;
  stories?: Record<string, Story>;
};
