import {createApi, fetchBaseQuery} from '@reduxjs/toolkit/query/react';
import {BASE_URL} from 'src/constants';
import type {Deployment} from 'src/models/Deployment';
import type {Exercise, NewExercise} from 'src/models/Exercise';

export const apiSlice = createApi({
  reducerPath: 'api',
  baseQuery: fetchBaseQuery({baseUrl: BASE_URL}),
  endpoints: builder => ({
    getExercises: builder.query<Exercise[], void>({
      query: () => '/exercise',
    }),
    getExercise: builder.query<Exercise, string>({
      query: exerciseId => `/exercise/${exerciseId}`,
    }),
    getDeployments: builder.query<Deployment[], string>({
      query: exerciseId => `/exercise/${exerciseId}/deployment`,
    }),
    addExercise: builder.mutation<Exercise, NewExercise>({
      query: newExercise => ({
        url: '/exercise', method: 'POST', body: newExercise,
      }),
    }),
    updateExercise: builder.mutation<Exercise, Exercise>({
      query: exercise => ({
        url: '/exercise', method: 'PUT', body: exercise,
      }),
    }),
  }),
});

export const {
  useGetExerciseQuery,
  useGetExercisesQuery,
  useGetDeploymentsQuery,
  useAddExerciseMutation,
  useUpdateExerciseMutation,
} = apiSlice;
