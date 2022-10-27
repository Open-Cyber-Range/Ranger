import React, {useState} from 'react';
import {Button, Dialog, H2, InputGroup} from '@blueprintjs/core';

const NameDialog = (
  {isOpen, title, placeholder, onSumbit, onCancel}:
  {
    isOpen: boolean;
    title: string;
    placeholder?: string;
    onSumbit: (name: string) => void;
    onCancel: () => void;
  },
) => {
  const [name, setName] = useState('');

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
        <InputGroup
          large
          value={name}
          leftIcon='graph'
          placeholder={placeholder ?? 'Name'}
          onChange={event => {
            setName(event.target.value);
          }}/>
      </div>
      <div className='bp4-dialog-footer'>
        <div className='bp4-dialog-footer-actions'>
          <Button
            large
            intent='primary'
            text='Add'
            onClick={() => {
              if (name !== '') {
                onSumbit(name);
                setName('');
              }
            }}/>
        </div>
      </div>
    </Dialog>

  );
};

export default NameDialog;
