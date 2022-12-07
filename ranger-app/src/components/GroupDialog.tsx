import React, {useState} from 'react';
import {
  Button,
  Dialog,
  H2,
  HTMLSelect,
  InputGroup,
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
  {isOpen, title, placeholder, onSumbit, onCancel}:
  {
    isOpen: boolean;
    title: string;
    placeholder?: string;
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
      <div className='bp4-dialog-header'>
        <H2>{title}</H2>
        <Button
          small
          minimal
          icon='cross'
          onClick={() => {
            onCancel();
          }}/>
      </div>
      <div className='bp4-dialog-body'>
        <div className='bp4-form-group .bp4-inline'>
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
        </div>
        <div className='bp4-form-group .bp4-inline'>
          <label className='bp4-label'>
            Deployement Group<span className='bp4-text-muted'> (Optional)</span>
          </label>
          <HTMLSelect
            autoFocus
            large
            options={GROUP_OPTIONS}
            value={group}
            placeholder={placeholder ?? 'Deployment Group'}
            onChange={event => {
              setGroup(event.target.value);
            }}
            onKeyDown={handleKeypress}/>
        </div>
        <InputGroup
          autoFocus
          large
          value={name}
          leftIcon='graph'
          placeholder={placeholder ?? 'Deployment Name'}
          onChange={event => {
            setName(event.target.value);
          }}
          onKeyDown={handleKeypress}/>
      </div>
      <div className='bp4-dialog-footer'>
        <div className='bp4-dialog-footer-actions'>
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
