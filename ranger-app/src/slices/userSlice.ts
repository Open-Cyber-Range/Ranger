import {type PayloadAction, createSlice} from '@reduxjs/toolkit';
import {type UserRole} from 'src/models/userRoles';
import {type RootState} from 'src/store';

const initialState: {
  token?: string;
  roles: UserRole[];
  selectedRole?: UserRole;
} = {
  roles: [],
  selectedRole: undefined,
};

export const userSlice = createSlice({
  name: 'user',
  initialState,
  reducers: {
    setToken(state, action: PayloadAction<string>) {
      state.token = action.payload;
    },
    setRoles(state, action: PayloadAction<UserRole[]>) {
      state.roles = action.payload;
    },
    selectRole(state, action: PayloadAction<UserRole | undefined>) {
      state.selectedRole = action.payload;
    },
  },
});

export const {setToken, setRoles, selectRole} = userSlice.actions;

export const selectedRoleSelector = (state: RootState) =>
  state.user.selectedRole;
export const rolesSelector = (state: RootState) => state.user.roles;
