import {
  Button,
  Card,
  Elevation,
  H5,
  MenuItem,
  type TreeNodeInfo,
} from '@blueprintjs/core';
import {Suggest2} from '@blueprintjs/select';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import React, {useEffect} from 'react';
import {
  useAdminAddParticipantMutation,
  useAdminGetDeploymentParticipantsQuery,
  useAdminGetDeploymentQuery,
  useAdminGetDeploymentScenarioQuery,
  useAdminGetGroupUsersQuery,
} from 'src/slices/apiSlice';
import {type AdUser} from 'src/models/groups';
import {MenuItem2} from '@blueprintjs/popover2';
import {useTranslation} from 'react-i18next';
import {toastSuccess, toastWarning} from 'src/components/Toaster';
import {createEntityTree} from 'src/utils';
import {type Participant} from 'src/models/pariticpant';

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

const filterList = (
  nonFilteredList: TreeNodeInfo[],
  participants: Participant[],
  initialList: TreeNodeInfo[] = [],
): TreeNodeInfo[] => {
  const selectors = new Set(participants.map(participant => participant.selector));
  for (const entity of Object.values(nonFilteredList)) {
    if (!selectors.has(entity.id as string)) {
      initialList.push(entity);
    }
  }

  return initialList;
};

const EntityConnector = ({exerciseId, deploymentId}: {
  exerciseId: string;
  deploymentId: string;
}) => {
  const {t} = useTranslation();
  const [addParticipant, {isSuccess, error}] = useAdminAddParticipantMutation();
  const {data: scenario} = useAdminGetDeploymentScenarioQuery({exerciseId, deploymentId});
  const {data: deployment} = useAdminGetDeploymentQuery({exerciseId, deploymentId});
  const {
    data: participants,
  } = useAdminGetDeploymentParticipantsQuery({exerciseId, deploymentId});
  const {data: users} = useAdminGetGroupUsersQuery(deployment?.groupName ?? skipToken);
  const [selectedUser, setSelectedUser] = React.useState<AdUser | undefined>(undefined);
  const [selectedEntity, setSelectedEntity] = React.useState<TreeNodeInfo | undefined>(undefined);

  const tree: TreeNodeInfo[] = React.useMemo(() => {
    if (!scenario?.entities || !participants) {
      return [];
    }

    const flattenedList = flattenList(createEntityTree(scenario.entities));
    return filterList(flattenedList, participants);
  }, [scenario, participants]);

  useEffect(() => {
    if (isSuccess) {
      toastSuccess(t('deployments.entityConnector.success'));
    }
  }
  , [isSuccess, t]);

  useEffect(() => {
    if (error) {
      toastWarning(t('deployments.entityConnector.fail'));
    }
  }
  , [error, t]);

  return (
    <Card elevation={Elevation.TWO}>
      <H5>{t('deployments.entityConnector.entityConnector')}</H5>
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
        <Button
          icon='confirm'
          intent='primary'
          onClick={async () => {
            if (selectedUser && selectedEntity) {
              await addParticipant({
                exerciseId,
                deploymentId,
                newParticipant: {
                  userId: selectedUser.id,
                  selector: selectedEntity.id.toString(),
                },
              });
              setSelectedEntity(undefined);
              setSelectedUser(undefined);
            }
          }}
        >{t('common.connect')}
        </Button>
      </div>
    </Card>
  );
};

export default EntityConnector;
