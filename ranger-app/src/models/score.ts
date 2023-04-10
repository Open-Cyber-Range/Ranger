export type Score = {
  id: string;
  exerciseId: string;
  deploymentId: string;
  metricName: string;
  value: number;
  timestamp: string;
  vmName: string;
  vmUuid: string;
};
