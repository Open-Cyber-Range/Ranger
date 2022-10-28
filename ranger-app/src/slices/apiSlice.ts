import {createApi, fetchBaseQuery} from '@reduxjs/toolkit/query/react';
import {BASE_URL} from 'src/constants';
import type {Deployment} from 'src/models/Deployment';
import type {Exercise} from 'src/models/Exercise';

export const apiSlice = createApi({
  reducerPath: 'api',
  baseQuery: fetchBaseQuery({baseUrl: BASE_URL}),
  endpoints: builder => ({
    getExercises: builder.query<Exercise[], void>({
      query: () => '/exercise',
    }),
    getDeployments: builder.query<Deployment[], string>({
      query: exerciseId => `/exercise/${exerciseId}/deployment`,
    }),
  }),
});

export const {useGetExercisesQuery, useGetDeploymentsQuery} = apiSlice;
