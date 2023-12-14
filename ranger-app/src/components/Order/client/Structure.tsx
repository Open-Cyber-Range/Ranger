import {
  Button,
  Callout,
  H5,
  Tree,
  type TreeNodeInfo,
} from '@blueprintjs/core';
import React, {useCallback, useEffect, useState} from 'react';
import {useTranslation} from 'react-i18next';
import {toastWarning} from 'src/components/Toaster';
import {type Order, type NewStructure, type Structure} from 'src/models/order';
import {
  useClientAddStructureMutation,
  useClientDeleteStructureMutation,
  useClientUpdateStructureMutation,
} from 'src/slices/apiSlice';
import {sortByProperty} from 'sort-by-property';
import StructureDialog from './StructureDialog';

const numberOfParents = (structure: Structure, structures: Structure[]): number => {
  if (structure.parentId === null) {
    return 0;
  }

  const parent = structures.find(s => s.id === structure.parentId);
  if (!parent) {
    return 0;
  }

  return 1 + numberOfParents(parent, structures);
};

const createStructureTree = (
  structures: Structure[],
  deleteFields: {
    text: string;
    callback: (structureId: string) => Promise<void>;
  },
  editFields: {
    text: string;
    callback: (structure: Structure) => Promise<void>;
  },
): TreeNodeInfo[] => {
  const {text: deleteText, callback: deleteStructureCallback} = deleteFields;
  const {text: editText, callback: editStructureCallback} = editFields;
  const initialStructure = [...structures].sort(sortByProperty('name', 'desc'));
  const sortedStructure: Structure[] = [];

  let backupCounter = 0;
  while (initialStructure.length > 0) {
    if (backupCounter > structures.length ** 2) {
      break;
    }

    const lastElement = initialStructure.pop();
    if (!lastElement) {
      break;
    }

    if (lastElement.parentId === null) {
      sortedStructure.push(lastElement);
    } else {
      const parentElement = sortedStructure
        .find(structure => structure.id === lastElement.parentId);
      if (parentElement) {
        sortedStructure.splice(sortedStructure.indexOf(parentElement) + 1, 0, lastElement);
      } else {
        initialStructure.unshift(lastElement);
      }
    }

    backupCounter += 1;
  }

  return sortedStructure.map(structure => ({
    id: structure.id,
    className: 'my-3',
    label: (
      <div
        className='flex'
        style={{
          paddingLeft: `${numberOfParents(structure, structures) * 1.1}rem`,
        }}
      >
        <H5>
          {structure.name}:
        </H5>
        <span className='ml-2'>
          {structure.description}
        </span>
      </div>
    ),
    secondaryLabel: (
      <div className='flex gap-2'>
        <Button
          className='pt-1'
          intent='warning'
          onClick={async () => {
            await editStructureCallback(structure);
          }}
        >
          {editText}
        </Button>
        <Button
          className='pt-1'
          intent='danger'
          onClick={async () => {
            await deleteStructureCallback(structure.id);
          }}
        >
          {deleteText}
        </Button>
      </div>
    ),
    nodeData: {
      structure,
      numberOfParents: numberOfParents(structure, structures),
    },
  }));
};

const StructureElement = ({order}: {order: Order}) => {
  const {t} = useTranslation();
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  const [addStructure, {error}] = useClientAddStructureMutation();
  const [deleteStructure, {error: deleteError}] = useClientDeleteStructureMutation();
  const [updateStructure, {error: updateError}]
      = useClientUpdateStructureMutation();
  const [editedStructure, setEditedStructure]
      = useState<Structure | undefined>();

  useEffect(() => {
    if (error) {
      toastWarning(t(
        'order.structureElements.failedtoAdd',
      ));
    }
  }, [error, t]);

  useEffect(() => {
    if (deleteError) {
      toastWarning(t(
        'order.structureElements.failedToDelete',
      ));
    }
  }, [deleteError, t]);

  useEffect(() => {
    if (updateError) {
      toastWarning(t(
        'order.structureElements.failedToUpdate',
      ));
    }
  }, [updateError, t]);

  const onHandleSubmit = async (formContent: NewStructure) => {
    if (editedStructure) {
      await updateStructure({
        newStructure: {
          ...editedStructure,
          ...formContent,
        },
        orderId: order.id,
        structureId: editedStructure.id,
      });
    } else {
      await addStructure({
        newStructure: formContent,
        orderId: order.id,
      });
    }

    setIsDialogOpen(false);
    setEditedStructure(undefined);
  };

  const handleDeleteStructure = useCallback(async (structureId: string) => {
    await deleteStructure({
      orderId: order.id,
      structureId,
    });
  }, [order.id, deleteStructure]);

  const handleEditStructure = useCallback(async (structure: Structure) => {
    setEditedStructure(structure);
    setIsDialogOpen(true);
  }, []);

  const tree: TreeNodeInfo[] = React.useMemo(() => {
    if (!order.structures) {
      return [];
    }

    return createStructureTree(
      order.structures,
      {
        text: t('orders.structureElements.delete'),
        callback: handleDeleteStructure,
      },
      {
        text: t('orders.structureElements.edit'),
        callback: handleEditStructure,
      },
    );
  }, [order.structures, t, handleDeleteStructure, handleEditStructure]);

  return (
    <>
      <StructureDialog
        order={order}
        crossClicked={() => {
          setIsDialogOpen(false);
        }}
        isOpen={isDialogOpen}
        editableStructure={editedStructure}
        onSubmit={onHandleSubmit}
      />
      <Callout intent='primary' icon='info-sign'>
        {t('orders.structureElements.explenation')}
      </Callout>
      <div className='mt-4 flex gap-4 justify-between items-start'>
        <div className='flex flex-col gap-4 grow'>
          <Tree
            contents={tree}
          />
        </div>
        <Button
          large
          className='shrink-0'
          intent='primary'
          onClick={() => {
            setEditedStructure(undefined);
            setIsDialogOpen(true);
          }}
        >
          {t('orders.structureElements.add')}
        </Button>
      </div>
    </>
  );
};

export default StructureElement;

