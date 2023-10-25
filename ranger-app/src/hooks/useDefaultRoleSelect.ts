import {useKeycloak} from '@react-keycloak/web';
import {useEffect} from 'react';
import {useDispatch, useSelector} from 'react-redux';
import {selectedRoleSelector, selectRole} from 'src/slices/userSlice';
import {UserRole} from 'src/models/userRoles';

const useDefaultRoleSelect = (): (UserRole | undefined) => {
  const currentRole = useSelector(selectedRoleSelector);
  const {keycloak} = useKeycloak();
  const dispatch = useDispatch();

  useEffect(() => {
    if (currentRole === undefined) {
      if (keycloak.hasRealmRole(UserRole.MANAGER.toString())) {
        dispatch(selectRole(UserRole.MANAGER));
      } else if (keycloak.hasRealmRole(UserRole.PARTICIPANT.toString())) {
        dispatch(selectRole(UserRole.PARTICIPANT));
      } else if (keycloak.hasRealmRole(UserRole.CLIENT.toString())) {
        dispatch(selectRole(UserRole.CLIENT));
      }
    }
  }, [currentRole, keycloak, dispatch, keycloak?.realmAccess?.roles]);

  return currentRole;
};

export default useDefaultRoleSelect;
