import {FormGroup, MenuItem} from '@blueprintjs/core';
import {MultiSelect} from '@blueprintjs/select';
import type React from 'react';
import {
  useFieldArray,
  type FieldValues,
  type ArrayPath,
  type Control,
  type FieldArrayWithId,
} from 'react-hook-form';

const DialogMultiSelect = <T extends FieldValues>({
  control,
  id,
  label,
  textRenderer,
  name,
  keyName,
  items,
}: {
  control: Control<T>;
  name: ArrayPath<T>;
  id: string;
  label: string;
  keyName: string;
  items: Array<FieldArrayWithId<T, ArrayPath<T>>>;
  textRenderer: (item: FieldArrayWithId<T, ArrayPath<T>>) => string;
}) => {
  const {
    fields,
    append,
    remove,
  } = useFieldArray({
    keyName,
    control,
    name,
  });

  return (
    <FormGroup
      labelFor={id}
      label={label}
    >
      <MultiSelect<FieldArrayWithId<T, ArrayPath<T>>>
        itemRenderer={item => (
          <MenuItem
            key={item.id}
            roleStructure='listoption'
            selected={fields.some(field => field.id === item.id)}
            disabled={fields.some(field => field.id === item.id)}
            shouldDismissPopover={false}
            text={textRenderer(item)}
            onClick={() => {
              append(item);
            }}
          />
        )}
        items={items}
        selectedItems={fields.map(field => ({...field, id: field.id.toString()}))}
        tagRenderer={textRenderer}
        onItemSelect={item => {
          append(item);
        }}
        onRemove={(_item, index) => {
          remove(index);
        }}
      />
    </FormGroup>
  );
};

export default DialogMultiSelect;

