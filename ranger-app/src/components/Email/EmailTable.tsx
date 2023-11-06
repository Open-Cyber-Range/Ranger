import React from 'react';
import {Callout, Intent, Tag} from '@blueprintjs/core';
import {useTranslation} from 'react-i18next';
import {useAdminGetEmailsQuery} from 'src/slices/apiSlice';
import {type Exercise} from 'src/models/exercise';
import {EmailStatusType} from 'src/models/email';

const emailIntent = (status: EmailStatusType): Intent => {
  switch (status) {
    case EmailStatusType.Sent: {
      return Intent.SUCCESS;
    }

    case EmailStatusType.Failed: {
      return Intent.WARNING;
    }

    default: {
      return Intent.PRIMARY;
    }
  }
};

const EmailTable = ({exercise}: {exercise: Exercise}) => {
  const {data: emails} = useAdminGetEmailsQuery(exercise.id);
  const {t} = useTranslation();

  if (emails) {
    return (
      <div>
        <table className='bp4-html-table bp4-html-table-striped'>
          <thead>
            <th>{t('emails.status')}</th>
            <th>{t('emails.timestamp')}</th>
            <th>{t('emails.from')}</th>
            <th>{t('emails.to')}</th>
            <th>{t('emails.replyTo')}</th>
            <th>{t('emails.cc')}</th>
            <th>{t('emails.bcc')}</th>
            <th>{t('emails.subject')}</th>
            <th>{t('emails.body')}</th>
          </thead>
          <tbody>
            {emails.map(email => (
              <tr key={email.id}>
                <td>
                  <Tag intent={emailIntent(email.statusType)}>
                    {email.statusType}
                  </Tag>
                </td>
                <td>{new Date(email.createdAt).toLocaleString()}</td>
                <td>{email.fromAddress}</td>
                <td>{email.toAddresses}</td>
                <td>{email.replyToAddresses}</td>
                <td>{email.ccAddresses}</td>
                <td>{email.bccAddresses}</td>
                <td>{email.subject}</td>
                <td>{email.body}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    );
  }

  return (
    <Callout title={t('emails.noEmails') ?? ''}/>
  );
};

export default EmailTable;
