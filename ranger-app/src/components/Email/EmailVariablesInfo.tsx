import React from 'react';
import {Button} from '@blueprintjs/core';
import {type EmailVariable} from 'src/models/email';
import {Tooltip2} from '@blueprintjs/popover2';
import {useTranslation} from 'react-i18next';

const EmailVariablesInfo = ({emailVariables}: {emailVariables: EmailVariable[]}) => {
  const {t} = useTranslation();
  return (
    <Tooltip2
      content={
        <div>
          <strong>{t('emails.variables.available')}</strong>
          <ul>
            {emailVariables.map(variable => (
              <li key={variable.name}>{variable.name} - {variable.description}</li>
            ))}
          </ul>
        </div>
      }
    >
      <Button minimal icon='info-sign'/>
    </Tooltip2>
  );
};

export default EmailVariablesInfo;
