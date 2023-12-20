import {Intent} from '@blueprintjs/core';
import {ElementStatus, type DeploymentElement} from 'src/models/deployment';
import {type Scenario} from 'src/models/scenario';

export const getProgressionAndStatus = (
  deploymentElements: DeploymentElement[],
  scenario: Scenario,
) => {
  let intentStatus: Intent = Intent.WARNING;
  let successfulElements = 0;
  const totalDeploymentElements = countElements(scenario);

  if (deploymentElements.length === 0 && totalDeploymentElements === 0) {
    return [1, Intent.SUCCESS] as const;
  }

  for (const element of deploymentElements) {
    const elementStatus = loadingIntent(element.status);

    if (elementStatus === Intent.DANGER) {
      return [1, Intent.DANGER] as const;
    }

    if (elementStatus === Intent.SUCCESS) {
      successfulElements += 1;
    }
  }

  const progression = successfulElements / totalDeploymentElements;

  if (progression >= 1) {
    intentStatus = Intent.SUCCESS;
  }

  return [progression, intentStatus] as const;
};

const loadingIntent = (status: ElementStatus): Intent => {
  switch (status) {
    case ElementStatus.ConditionSuccess:
    case ElementStatus.ConditionPolling:
    case ElementStatus.ConditionClosed:
    case ElementStatus.Success:
    case ElementStatus.Removed: {
      return Intent.SUCCESS;
    }

    case ElementStatus.Ongoing: {
      return Intent.WARNING;
    }

    case ElementStatus.Failed:
    case ElementStatus.RemoveFailed: {
      return Intent.DANGER;
    }

    default: {
      return Intent.WARNING;
    }
  }
};

function countElements(scenario: Scenario) {
  let totalElements = 0;
  const templates: string[] = [];

  if (scenario.infrastructure && Object.keys(scenario.infrastructure).length > 0) {
    for (const infraNode of Object.values(scenario.infrastructure)) {
      totalElements += infraNode.count;
    }
  }

  if (scenario.nodes && Object.keys(scenario.nodes).length > 0) {
    for (const node of Object.values(scenario.nodes)) {
      if (node.source && !templates.includes(node.source.name)) {
        templates.push(node.source.name);
        totalElements += 1;
      }

      if (node.features && Object.keys(node.features).length > 0) {
        totalElements += Object.keys(node.features).length;
      }

      if (node.conditions && Object.keys(node.conditions).length > 0) {
        totalElements += Object.keys(node.conditions).length;
      }
    }
  }

  return totalElements;
}
