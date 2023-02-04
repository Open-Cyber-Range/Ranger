import React from 'react';
import {Tag} from '@blueprintjs/core';
import type {DeploymentElement} from 'src/models/deployment';
import {DeployerType} from 'src/models/deployment';
import styled from 'styled-components';
import '@blueprintjs/popover2/lib/css/blueprint-popover2.css';
import {useTranslation} from 'react-i18next';

const TagWrapper = styled.div`
  display: flex;
  margin: 2px;
  margin-top: auto;
  margin-bottom: 1rem;
  
`;

const countNodesByType = (deploymentElements: DeploymentElement[]) => {
  let [virtualMachines, switches, templates] = [0, 0, 0];

  for (const element of deploymentElements) {
    switch (element.deployerType) {
      case (DeployerType.VirtualMachine): {
        virtualMachines += 1;
        break;
      }

      case (DeployerType.Switch): {
        switches += 1;
        break;
      }

      case (DeployerType.Template): {
        templates += 1;
        break;
      }

      default: {
        break;
      }
    }
  }

  return [virtualMachines, switches, templates];
};

const InfoTag = ({name, count}: {name: string; count: number}) => (
  <TagWrapper>
    <Tag>{name}: {count}</Tag>
  </TagWrapper>
);

const InfoTags = (
  {deploymentElements}: {deploymentElements: DeploymentElement[]}) => {
  const [
    virtualMachineCount,
    switchCount,
    templateCount,
  ] = countNodesByType(deploymentElements);
  const {t} = useTranslation();
  return (
    <>
      <InfoTag name={t('common.virtualMachines')} count={virtualMachineCount}/>
      <InfoTag name={t('common.switches')} count={switchCount}/>
      <InfoTag name={t('common.templates')} count={templateCount}/>
    </>
  );
};

export default InfoTags;
