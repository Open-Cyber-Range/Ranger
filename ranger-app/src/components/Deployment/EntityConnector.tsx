import {
  MenuItem,
  type TreeNodeInfo,
  Card,
  Elevation,
  H5,
  Button,
} from '@blueprintjs/core';
import {Suggest2} from '@blueprintjs/select';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import React from 'react';
import {type Entity} from 'src/models/scenario';
import {
  useAdminGetDeploymentQuery,
  useAdminGetDeploymentScenarioQuery,
  useAdminGetGroupUsersQuery,
} from 'src/slices/apiSlice';
import {type AdUser} from 'src/models/groups';
import {MenuItem2} from '@blueprintjs/popover2';
import {useTranslation} from 'react-i18next';

const createEntityTree = (
  entities: Record<string, Entity>, selector?: string,
): TreeNodeInfo[] => {
  const tree: TreeNodeInfo[] = [];
  for (const [entityId, entity] of Object.entries(entities)) {
    const id = selector ? `${selector}.${entityId}` : entityId;
    const entityNode: TreeNodeInfo = {
      id,
      label: entity.name ?? entityId,
      icon: 'new-person',
      isExpanded: true,
    };
    if (entity.entities) {
      const subtree = createEntityTree(entity.entities, id);
      entityNode.childNodes = subtree;
    }

    tree.push(entityNode);
  }

  return tree;
};

const flattenList = (
  nonFlattenedList: TreeNodeInfo[], initialList: TreeNodeInfo[] = [],
): TreeNodeInfo[] => {
  for (const item of nonFlattenedList) {
    initialList.push(item);
    if (item.childNodes) {
      flattenList(item.childNodes, initialList);
    }
  }

  return initialList;
};

const EntityConnector = ({exerciseId, deploymentId}: {
  exerciseId: string;
  deploymentId: string;
}) => {
  const {t} = useTranslation();
  const {data: scenario} = useAdminGetDeploymentScenarioQuery({exerciseId, deploymentId});
  const {data: deployment} = useAdminGetDeploymentQuery({exerciseId, deploymentId});
  const {data: users} = useAdminGetGroupUsersQuery(deployment?.groupName ?? skipToken);
  const [selectedUser, setSelectedUser] = React.useState<AdUser | undefined>(undefined);
  const [selectedEntity, setSelectedEntity] = React.useState<TreeNodeInfo | undefined>(undefined);

  const tree: TreeNodeInfo[] = React.useMemo(() => {
    if (!scenario?.entities) {
      return [];
    }

    return flattenList(createEntityTree(scenario.entities));
  }, [scenario]);

  return (
    <Card elevation={Elevation.TWO}>
      <H5>Entity Connector</H5>
      <div className='grid grid-cols-2 gap-2'>
        <Suggest2<TreeNodeInfo>
          inputProps={{
            placeholder: t('deployments.entityConnector.selectEntity') ?? '',
          }}
          activeItem={selectedEntity ?? null}
          inputValueRenderer={item => item.id.toString() ?? ''}
          itemPredicate={(query, item) =>
            item.id.toString().toLowerCase().includes(query.toLowerCase()) ?? false}
          itemRenderer={(item, {handleClick, handleFocus}) => (
            <MenuItem2
              key={item.id}
              style={{
                paddingLeft: `${Number(item.id.toString().split('.').length) * 0.5}rem`,
              }}
              text={item.id.toString().split('.').pop() ?? ''}
              onClick={handleClick}
              onFocus={handleFocus}
            />
          )}
          items={tree ?? []}
          noResults={
            <MenuItem
              disabled
              text={t('common.noResults')}
              roleStructure='listoption'/>
          }
          onItemSelect={item => {
            setSelectedEntity(item);
          }}
        />

        <Suggest2<AdUser>
          inputProps={{
            placeholder: t('deployments.entityConnector.selectUser') ?? '',
          }}
          activeItem={selectedUser ?? null}
          inputValueRenderer={item => item.username ?? ''}
          itemPredicate={(query, item) =>
            item.username?.toLowerCase().includes(query.toLowerCase()) ?? false}
          itemRenderer={(item, {handleClick, handleFocus}) => (
            <MenuItem2
              key={item.id}
              text={item.username}
              onClick={handleClick}
              onFocus={handleFocus}
            />
          )}
          items={users ?? []}
          noResults={
            <MenuItem
              disabled
              text={t('common.noResults')}
              roleStructure='listoption'/>
          }
          onItemSelect={item => {
            setSelectedUser(item);
          }}
        />
      </div>
      <div className='py-[1rem] flex justify-end'>
        <Button icon='confirm' intent='primary'>{t('common.connect')}</Button>
      </div>
    </Card>
  );
};

export default EntityConnector;
