import React, {useEffect} from 'react';
import {Alignment, Button, Navbar} from '@blueprintjs/core';
import {useTranslation} from 'react-i18next';
import {useKeycloak} from '@react-keycloak/web';
import {setToken} from 'src/slices/userSlice';
import {useDispatch} from 'react-redux';
import RoleSelect from './RoleSelect';

const LoginInfo = () => {
  const {t} = useTranslation();
  const {keycloak} = useKeycloak();
  const {token} = keycloak;
  const dispatch = useDispatch();

  useEffect(() => {
    if (token !== undefined) {
      dispatch(setToken(token));
    }
  }, [token, dispatch]);

  useEffect(() => {
    keycloak.onTokenExpired = () => {
      keycloak.updateToken(180).then(refreshed => {
        if (refreshed && keycloak.token) {
          dispatch(setToken(keycloak.token));
        }
      }).catch(() => {
        // No need to do anything here, the token will be refreshed
      });
    };
  }, [
    keycloak,
    keycloak.token,
    keycloak.onTokenExpired,
    keycloak.updateToken,
    dispatch,
  ]);

  return (
    <Navbar.Group align={Alignment.RIGHT}>
      {keycloak.authenticated && (
        <>
          {keycloak.tokenParsed?.preferred_username !== undefined && (
            <Navbar.Heading className='hidden md:block'>

              {t('menu.greeting',
                {
                  username: keycloak
                    .tokenParsed?.preferred_username as string,
                })}
            </Navbar.Heading>
          )}
          <Navbar.Heading>
            <RoleSelect keycloak={keycloak}/>
          </Navbar.Heading>
          <Button
            minimal
            icon='log-out'
            onClick={async () => keycloak.logout()}
          >{t('menu.logout')}
          </Button>
        </>
      )}
    </Navbar.Group>
  );
};

export default LoginInfo;
