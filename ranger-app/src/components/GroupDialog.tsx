import React, {useState} from 'react';
import {
  Button,
  Dialog,
  H2,
  HTMLSelect,
  InputGroup,
  FormGroup,
  Classes,
  Label,
} from '@blueprintjs/core';
import {useTranslation} from 'react-i18next';

const GROUP_OPTIONS = [
  'Group List',
  'Group - 1',
  'Group - 2',
  'Group - 3',
  'Group - 4',
];

const GroupDialog = (
  {isOpen, title, onSumbit, onCancel}:
  {
    isOpen: boolean;
    title: string;
    onSumbit: (count: string, name: string, group: string) => void;
    onCancel: () => void;
  },
) => {
  const {t} = useTranslation();
  const [name, setName] = useState('');
  const [group, setGroup] = useState('');
  const [count, setCount] = useState('1');

  const handleKeypress = (event: {key: string}) => {
    if (event.key === 'Enter' && name !== '') {
      onSumbit(name, count, group);
      setName('');
    }
  };

  return (
    <Dialog isOpen={isOpen}>
      <div className={Classes.DIALOG_HEADER}>
        <H2>{title}</H2>
        <Button
          small
          minimal
          icon='cross'
          onClick={() => {
            onCancel();
          }}/>
      </div>
      <div className={Classes.DIALOG_BODY}>
        <FormGroup>
          <InputGroup
            autoFocus
            large
            value={count}
            leftIcon='array-numeric'
            onChange={event => {
              setCount(event.target.value);
            }}
            onKeyDown={handleKeypress}
          />
        </FormGroup>
        <FormGroup>
          <Label>
            {t('Deployment Group (Optional)')}
            <HTMLSelect
              autoFocus
              large
              fill
              options={GROUP_OPTIONS}
              value={group}
              onChange={event => {
                setGroup(event.target.value);
              }}
              onKeyDown={handleKeypress}/>
          </Label>
        </FormGroup>
        <FormGroup>
          <Label>
            {t('Deployment Name')}
            <InputGroup
              autoFocus
              large
              value={name}
              leftIcon='graph'
              onChange={event => {
                setName(event.target.value);
              }}
              onKeyDown={handleKeypress}/>
          </Label>
        </FormGroup>
      </div>
      <div className={Classes.DIALOG_FOOTER}>
        <div className={Classes.DIALOG_FOOTER_ACTIONS}>
          <Button
            large
            intent='primary'
            text={t('common.add')}
            onClick={() => {
              if (name !== '') {
                onSumbit(name, count, group);
                setName('');
              }
            }}/>
        </div>
      </div>
    </Dialog>

  );
};

export default GroupDialog;
