import {H1} from '@blueprintjs/core';
import React from 'react';
import {useTranslation} from 'react-i18next';
import EmailTable from 'src/components/EmailTable';
import PageHolder from 'src/components/PageHolder';

const EmailLog = () => {
  const {t} = useTranslation();
  return (
    <PageHolder>
      <H1>{t('emails.emailLog')}</H1>
      <EmailTable/>
    </PageHolder>
  );
};

export default EmailLog;
