import React, {useEffect} from 'react';
import {useTranslation} from 'react-i18next';
import {useDispatch, useSelector} from 'react-redux';
import {type KeycloakInstance as Keycloak} from 'keycloak-js';
import {UserRole} from 'src/models/userRoles';
import {
  rolesSelector,
  selectRole,
  selectedRoleSelector,
  setRoles,
} from 'src/slices/userSlice';

const RoleSelect = ({keycloak}: {keycloak: Keycloak}) => {
  const {t} = useTranslation();
  const currentRole = useSelector(selectedRoleSelector);
  const roles = useSelector(rolesSelector);
  const dispatch = useDispatch();

  useEffect(() => {
    const newRoles: UserRole[] = [];
    if (keycloak.hasRealmRole(UserRole.MANAGER.toString())) {
      newRoles.push(UserRole.MANAGER);
    }

    if (keycloak.hasRealmRole(UserRole.PARTICIPANT.toString())) {
      newRoles.push(UserRole.PARTICIPANT);
    }

    dispatch(setRoles(newRoles));
  }, [keycloak, dispatch]);

  useEffect(() => {
    if (currentRole === undefined) {
      if (keycloak.hasRealmRole(UserRole.MANAGER.toString())) {
        dispatch(selectRole(UserRole.MANAGER));
      } else if (keycloak.hasRealmRole(UserRole.PARTICIPANT.toString())) {
        dispatch(selectRole(UserRole.PARTICIPANT));
      }
    }
  }, [currentRole, keycloak, dispatch]);

  return (
    <div className='
    bp4-html-select
    bp4-minimal
    bp4-large'
    >
      <select
        disabled={roles.length < 2}
        value={currentRole?.toString() ?? 'no-role'}
        onChange={event => {
          const eventValue = event.target.value;

          const newRole = eventValue === 'no-role'
            ? undefined : eventValue as UserRole;
          dispatch(selectRole(newRole));
        }}
      >
        {Object
          .entries(UserRole)
          .map(
            ([key, value]) => {
              if (roles.includes(value as UserRole)) {
                return (
                  <option
                    key={key}
                    value={value}
                  >
                    {t(`menu.userRoles.${value}`)}
                  </option>
                );
              }

              return null;
            })}
        <option className='hidden' value='no-role'>{t('menu.noRole')}</option>
      </select>
      <span className='bp4-icon bp4-icon-double-caret-vertical'/>
    </div>

  );
};

export default RoleSelect;
