import {type PayloadAction, createSlice} from '@reduxjs/toolkit';

const initialState: {token?: string} = {};

export const tokenSlice = createSlice({
  name: 'token',
  initialState,
  reducers: {
    setToken(state, action: PayloadAction<string>) {
      state.token = action.payload;
    },
  },
});

export const {setToken} = tokenSlice.actions;
