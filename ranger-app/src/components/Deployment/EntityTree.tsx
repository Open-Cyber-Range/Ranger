import {
  type TreeNodeInfo,
  Card,
  Elevation,
  H5,
  Tree,
} from '@blueprintjs/core';
import React from 'react';
import {
  useAdminGetDeploymentParticipantsQuery,
  useAdminGetDeploymentQuery,
  useAdminGetDeploymentScenarioQuery,
  useAdminGetGroupUsersQuery,
} from 'src/slices/apiSlice';
import {useTranslation} from 'react-i18next';
import {createEntityTree} from 'src/utils';
import {skipToken} from '@reduxjs/toolkit/dist/query';

const EntityTree = ({exerciseId, deploymentId}: {
  exerciseId: string;
  deploymentId: string;
}) => {
  const {t} = useTranslation();
  const {data: scenario} = useAdminGetDeploymentScenarioQuery({exerciseId, deploymentId});
  const {data: deployment} = useAdminGetDeploymentQuery({exerciseId, deploymentId});
  const {data: users} = useAdminGetGroupUsersQuery(deployment?.groupName ?? skipToken);
  const {
    data: particpants,
  } = useAdminGetDeploymentParticipantsQuery({exerciseId, deploymentId});

  const tree: TreeNodeInfo[] = React.useMemo(() => {
    if (!scenario?.entities) {
      return [];
    }

    return createEntityTree(scenario.entities, particpants, users);
  }, [scenario, users, particpants]);

  return (
    <Card elevation={Elevation.TWO}>
      <H5>{t('deployments.entityTree')}</H5>
      <Tree
        contents={tree}
      />
    </Card>
  );
};

export default EntityTree;

