import React from 'react';
import {Intent, ProgressBar} from '@blueprintjs/core';
import type {Deployment, DeploymentElement} from 'src/models/deployment';
import '@blueprintjs/popover2/lib/css/blueprint-popover2.css';
import {ElementStatus} from 'src/models/deployment';

const loadingIntent = (status: ElementStatus): Intent => {
  switch (status) {
    case ElementStatus.Success:
    case ElementStatus.Removed:
      return Intent.SUCCESS;
    case ElementStatus.Ongoing:
      return Intent.WARNING;
    case ElementStatus.Failed:
    case ElementStatus.RemoveFailed:
      return Intent.DANGER;
    default:
      return Intent.WARNING;
  }
};

const getProgressionAndStatus = (
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

const StatusBar = (
  {deployment, deploymentElements}: {
    deployment: Deployment;
    deploymentElements: DeploymentElement[];
  },
) => {
  const [loadingValue, intent] = getProgressionAndStatus(deploymentElements);
  return (
    <ProgressBar
      key={deployment.id}
      value={loadingValue}
      animate={loadingValue < 0.999}
      stripes={loadingValue < 0.999}
      intent={intent}
    />
  );
};

export default StatusBar;
