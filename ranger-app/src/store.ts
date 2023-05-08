import {configureStore} from '@reduxjs/toolkit';
import {apiSlice} from 'src/slices/apiSlice';
import {tokenSlice} from 'src/slices/tokenSlice';
import {useDispatch} from 'react-redux';

const store = configureStore({
  reducer: {
    [apiSlice.reducerPath]: apiSlice.reducer,
    [tokenSlice.name]: tokenSlice.reducer,
  },
  middleware: getDefaultMiddleware =>
    getDefaultMiddleware().concat(apiSlice.middleware),
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
export const useAppDispatch: () => AppDispatch = useDispatch;

export default store;
