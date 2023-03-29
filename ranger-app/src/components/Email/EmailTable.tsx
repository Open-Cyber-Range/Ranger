import {Intent, Tag} from '@blueprintjs/core';
import React from 'react';
import {useTranslation} from 'react-i18next';
import type {Email} from 'src/models/exercise';
import {EmailStatus} from 'src/models/exercise';

const dummyEmails: Email[] = [
  {
    id: '1',
    exerciseId: '21',
    toEntity: '11',
    fromAddress: 'liisa.sakerman@ocr.cr14.net',
    to: 'test@test.com',
    replyTo: 'liisa.sakerman@ocr.cr14.net',
    subject: 'subject',
    cc: 'test@test.com',
    bcc: 'test@test.com',
    body: 'body',
    status: EmailStatus.Delivered,
    sentAt: '24.01.2023',
  },
  {
    id: '2',
    exerciseId: '21',
    toEntity: '12',
    fromAddress: 'liisi.pal@ocr.cr14.net',
    to: 'test@test.com',
    subject: 'question',
    body: 'Short letter',
    status: EmailStatus.Bounced,
    sentAt: '25.01.2023',
  },
  {
    id: '3',
    exerciseId: '22',
    fromAddress: 'first.last@ocr.cr14.net',
    toEntity: '13',
    to: 'test@test.com',
    replyTo: 'fist.last@ocr.cr14.net',
    subject: 'Letter to report an issue about the excersise',
    bcc: 'test@test.com',
    body: 'long letter with lots of information about different random matters',
    status: EmailStatus.BeingSent,
    sentAt: '26.01.2023',
  },
];

const emailIntent = (status: EmailStatus): Intent => {
  switch (status) {
    case EmailStatus.Delivered: {
      return Intent.SUCCESS;
    }

    case EmailStatus.Bounced: {
      return Intent.WARNING;
    }

    default: {
      return Intent.PRIMARY;
    }
  }
};

const EmailTable = () => {
  const {t} = useTranslation();
  return (
    <div>
      <table className='bp4-html-table bp4-html-table-striped'>
        <thead>
          <th>{t('emails.status')}</th>
          <th>{t('emails.timestamp')}</th>
          <th>{t('emails.toEntity')}</th>
          <th>{t('emails.from')}</th>
          <th>{t('emails.to')}</th>
          <th>{t('emails.replyTo')}</th>
          <th>{t('emails.subject')}</th>
          <th>{t('emails.bcc')}</th>
          <th>{t('emails.cc')}</th>
          <th>{t('emails.body')}</th>
        </thead>
        <tbody>
          {dummyEmails.map(email => (
            <tr key={email.id}>
              <td>
                <Tag intent={emailIntent(email.status)}>
                  {email.status.charAt(0).toUpperCase() + email.status.slice(1)}
                </Tag>
              </td>
              <td>{email.sentAt}</td>
              <td>{email.toEntity}</td>
              <td>{email.fromAddress}</td>
              <td>{email.to}</td>
              <td>{email.replyTo}</td>
              <td>{email.subject}</td>
              <td>{email.bcc}</td>
              <td>{email.cc}</td>
              <td>{email.body}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

export default EmailTable;
