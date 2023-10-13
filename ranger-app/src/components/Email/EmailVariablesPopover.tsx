import type React from 'react';
import {Button, Menu, MenuItem} from '@blueprintjs/core';
import {Popover2} from '@blueprintjs/popover2';
import {type EmailVariable} from 'src/models/email';
import {useTranslation} from 'react-i18next';

type EmailVariablesProps = {
  emailVariables: EmailVariable[];
  insertVariable: (variableName: string) => void;
};

const EmailVariablesMenu = ({emailVariables, insertVariable}: EmailVariablesProps) => (
  <Menu>
    {emailVariables.map(variable => (
      <MenuItem
        key={variable.name}
        text={variable.description}
        onClick={() => {
          insertVariable(variable.name);
        }}
      />
    ))}
  </Menu>
);

const EmailVariablesPopover = ({emailVariables, insertVariable}: EmailVariablesProps) => {
  const {t} = useTranslation();

  return (
    <Popover2
      content={<EmailVariablesMenu
        emailVariables={emailVariables}
        insertVariable={insertVariable}/>}
      position='bottom-left'
    >
      <Button minimal icon='insert' text={t('emails.variables.insert')}/>
    </Popover2>
  );
};

export default EmailVariablesPopover;
