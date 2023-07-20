import {Classes, MenuItem, Tree, type TreeNodeInfo} from '@blueprintjs/core';
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
      const subtree = createEntityTree(entity.entities, selector);
      entityNode.childNodes = subtree;
    }

    tree.push(entityNode);
  }

  return tree;
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

  const tree: TreeNodeInfo[] = React.useMemo(() => {
    if (!scenario?.entities) {
      return [];
    }

    return createEntityTree(scenario.entities);
  }, [scenario]);

  return (
    <div>
      <h1>Entity Connector</h1>
      <div className='grid grid-cols-2 gap-2'>
        <Tree className={Classes.ELEVATION_0} contents={tree}/>
        <div>
          <Suggest2<AdUser>
            inputProps={{
              id: 'deployment-group',
              placeholder: '',
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
      </div>
    </div>
  );
};

export default EntityConnector;
