import {type Source} from './common';

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

export type Nodes = Record<string, Node>;
