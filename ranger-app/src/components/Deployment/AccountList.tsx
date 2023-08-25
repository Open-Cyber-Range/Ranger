import React from 'react';
import {useTranslation} from 'react-i18next';
import {skipToken} from '@reduxjs/toolkit/query';
import {H3} from '@blueprintjs/core';
import {useAdminGetDeploymentUsersQuery} from 'src/slices/apiSlice';

const AccountList = ({exerciseId, deploymentId}:
{exerciseId: string;
  deploymentId: string;
}) => {
  const {t} = useTranslation();
  const {data: users} = useAdminGetDeploymentUsersQuery(
    exerciseId && deploymentId ? {exerciseId, deploymentId} : skipToken);

  if (users && users.length > 0) {
    return (
      <div className='flex flex-col mt-8'>
        <H3>Accounts</H3>
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
    <div className='
      flex justify-center align-center m-2 mt-10 mb-auto text-gray-400'
    >
      {t('deployments.noAccounts')}
    </div>
  );
};

export default AccountList;