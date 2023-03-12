import {type Source} from './common';

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

export type Features = Record<string, Feature>;
