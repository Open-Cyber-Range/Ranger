import {createApi, fetchBaseQuery} from '@reduxjs/toolkit/query/react';
import {BASE_URL} from 'src/constants';
import type {
  Deployers,
  Deployment,
  DeploymentElement,
  NewDeployment,
} from 'src/models/deployment';
import type {Exercise, NewExercise, UpdateExercise} from 'src/models/exercise';

export const apiSlice = createApi({
  reducerPath: 'api',
  baseQuery: fetchBaseQuery({baseUrl: BASE_URL}),
  tagTypes: ['Deployment', 'Exercise'],
  endpoints: builder => ({
    getExercises: builder.query<Exercise[], void>({
      query: () => '/exercise',
      providesTags: (result = []) =>
        [
          ...result.map(({id}) => ({type: 'Exercise' as const, id})),
          {type: 'Exercise', id: 'LIST'},
        ],
    }),
    getExercise: builder.query<Exercise, string>({
      query: exerciseId => `/exercise/${exerciseId}`,
      providesTags: (result, error, id) => [{type: 'Exercise', id}],
    }),
    addExercise: builder.mutation<Exercise, NewExercise>({
      query: newExercise => ({
        url: '/exercise', method: 'POST', body: newExercise,
      }),
      invalidatesTags: [{type: 'Exercise', id: 'LIST'}],
    }),
    updateExercise: builder.mutation<Exercise, {
      exerciseUpdate: UpdateExercise; exerciseId: string;
    }>({
      query: ({exerciseUpdate, exerciseId}) => ({
        url: `/exercise/${exerciseId}`, method: 'PUT', body: exerciseUpdate,
      }),
      invalidatesTags: (result, error, {exerciseId}) =>
        [{type: 'Exercise', id: exerciseId}],
    }),
    getDeployments: builder.query<Deployment[], string>({
      query: exerciseId => `/exercise/${exerciseId}/deployment`,
      providesTags: (result = []) =>
        [
          ...result.map(({id}) => ({type: 'Deployment' as const, id})),
          {type: 'Deployment', id: 'LIST'},
        ],
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
      invalidatesTags: [{type: 'Deployment', id: 'LIST'}],
    }),
    deleteDeployment: builder
      .mutation<string, {exerciseId: string; deploymentId: string}>({
      query: ({exerciseId, deploymentId}) => ({
        url: `/exercise/${exerciseId}/deployment/${deploymentId}`,
        method: 'DELETE',
        responseHandler: async response => response.text(),

      }),
      invalidatesTags: (result, error, {deploymentId}) =>
        [{type: 'Deployment', id: deploymentId}],
    }),
    getDeploymentElements: builder
      .query<DeploymentElement[], {exerciseId: string; deploymentId: string}>({
      query: ({exerciseId, deploymentId}) =>
        `/exercise/${exerciseId}/deployment/${deploymentId}/deployment_element`,
    }),
    getDeploymentGroups: builder.query<Deployers, void>({
      query: () => '/deployer',
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
  useDeleteDeploymentMutation,
  useGetDeploymentElementsQuery,
  useGetDeploymentGroupsQuery,
} = apiSlice;
