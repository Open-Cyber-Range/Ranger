import React from 'react';
import {useTranslation} from 'react-i18next';
import {Callout, H3} from '@blueprintjs/core';
import {type AdUser} from 'src/models/groups';

const AccountList = ({users}:
{users: AdUser[] | undefined;
}) => {
  const {t} = useTranslation();

  if (users && users.length > 0) {
    return (
      <div className='flex flex-col mt-8'>
        <H3 className='text-center'>Accounts</H3>
        <div className='flex flex-col'>
          <table className='
              bp4-html-table
              bp4-html-table-striped'
          >
            <thead>
              <tr className='flex flex-row justify-between'>
                <th>Username</th>
                <th>Password</th>
                <th>Private Key</th>
              </tr>
            </thead>
            <tbody>
              {users.map(adUser => (
                adUser.accounts.map(account => (
                  <tr key={account.username}>
                    <td className='flex flex-row justify-between'>
                      <p className='mb-0'>{account.username}</p>
                      <p className='mb-0'>{account.password}</p>
                      <p className='mb-0'>{account.privateKey}</p>
                    </td>
                  </tr>
                ))
              ))}
            </tbody>
          </table>
        </div>
      </div>
    );
  }

  return (
    <Callout title={t('deployments.noAccounts') ?? ''}/>
  );
};

export default AccountList;
