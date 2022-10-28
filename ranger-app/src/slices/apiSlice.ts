import {createApi, fetchBaseQuery} from '@reduxjs/toolkit/query/react';
import {BASE_URL} from 'src/constants';
import type {Deployment, NewDeployment} from 'src/models/Deployment';
import type {Exercise, NewExercise, UpdateExercise} from 'src/models/Exercise';

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
    addExercise: builder.mutation<Exercise, NewExercise>({
      query: newExercise => ({
        url: '/exercise', method: 'POST', body: newExercise,
      }),
    }),
    updateExercise: builder.mutation<Exercise, {
      exerciseUpdate: UpdateExercise; exerciseId: string;
    }>({
      query: ({exerciseUpdate, exerciseId}) => ({
        url: `/exercise/${exerciseId}`, method: 'PUT', body: exerciseUpdate,
      }),
    }),
    getDeployments: builder.query<Deployment[], string>({
      query: exerciseId => `/exercise/${exerciseId}/deployment`,
    }),
    addDeployment: builder
      .mutation<Deployment,
    {
      newDeployment: NewDeployment; exerciseId: string;
    }>({
      query: ({newDeployment, exerciseId}) => ({
        url: `/exercise/${exerciseId}/deployment`,
        method: 'POST',
        body: newDeployment,
      }),
    }),
  }),
});

export const {
  useGetExerciseQuery,
  useGetExercisesQuery,
  useAddExerciseMutation,
  useUpdateExerciseMutation,
  useGetDeploymentsQuery,
  useAddDeploymentMutation,
} = apiSlice;
