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
  entities: Record<string, Entity>, counter = 0,
): [TreeNodeInfo[], number] => {
  const tree: TreeNodeInfo[] = [];
  for (const [entityId, entity] of Object.entries(entities)) {
    const entityNode: TreeNodeInfo = {
      id: entityId,
      label: entity.name ?? entityId,
      icon: 'new-person',
      isExpanded: true,
    };
    if (entity.entities) {
      const [subTree, newCount] = createEntityTree(entity.entities, counter);
      entityNode.childNodes = subTree;
      counter = newCount;
    }

    counter += 1;
    tree.push(entityNode);
  }

  return [tree, counter];
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

  const [tree, _] = React.useMemo(() => {
    if (!scenario?.entities) {
      return [[], 0];
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
            inputValueRenderer={item => item.username}
            itemPredicate={(query, item) =>
              item.username.toLowerCase().includes(query.toLowerCase())}
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
