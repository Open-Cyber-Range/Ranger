import React, {useState} from 'react';
import {Button, Dialog, H2, InputGroup} from '@blueprintjs/core';
import {useTranslation} from 'react-i18next';

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
  const {t} = useTranslation();
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
          autoFocus
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
            text={t('common.add')}
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
