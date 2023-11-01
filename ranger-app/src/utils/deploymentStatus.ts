import {Intent} from '@blueprintjs/core';
import {ElementStatus, type DeploymentElement} from 'src/models/deployment';

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

export const getProgressionAndStatus = (
  deploymentElements: DeploymentElement[],
) => {
  let loadingBarValue = 0;
  let intentStatus: Intent = Intent.WARNING;

  for (const element of deploymentElements) {
    if (element.status !== ElementStatus.Ongoing) {
      loadingBarValue += (1 / deploymentElements.length);
    }

    if (intentStatus !== Intent.DANGER) {
      intentStatus = loadingIntent(element.status);
    }
  }

  return [loadingBarValue, intentStatus] as const;
};
