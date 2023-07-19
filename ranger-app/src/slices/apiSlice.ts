/* eslint-disable max-len */
import {createApi, fetchBaseQuery} from '@reduxjs/toolkit/query/react';
import {BASE_URL} from 'src/constants';
import type {
  Deployers,
  Deployment,
  DeploymentElement,
  NewDeployment,
  ParticipantDeployment,
} from 'src/models/deployment';
import {
  type ParticipantExercise,
  type EmailForm,
  type Exercise,
  type NewExercise,
  type UpdateExercise,
  type Participant,
} from 'src/models/exercise';
import {type AdGroup, type AdUser} from 'src/models/groups';
import {type Scenario} from 'src/models/scenario';
import {type Score} from 'src/models/score';
import {type RootState} from 'src/store';

export const apiSlice = createApi({
  reducerPath: 'api',
  baseQuery: fetchBaseQuery({
    baseUrl: BASE_URL,
    prepareHeaders(headers, {getState}) {
      const token = (getState() as RootState).user.token;
      if (token) {
        headers.set('authorization', `Bearer ${token}`);
      }

      return headers;
    },
  }),
  tagTypes: ['Deployment', 'Exercise', 'Score', 'Scenario'],
  endpoints: builder => ({
    adminGetGroups: builder.query<AdGroup[], void>({
      query: () => '/admin/group',
    }),
    adminGetGroupUsers: builder.query<AdUser[], string>({
      query: groupName => `/admin/group/${groupName}/users`,
    }),
    adminGetDeploymentUsers: builder.query<AdUser[], {deploymentId: string; exerciseId: string}>({
      query: ({deploymentId, exerciseId}) => ({
        url: `/admin/exercise/${exerciseId}/deployment/${deploymentId}/users`,
      }),
    }),
    adminGetExercises: builder.query<Exercise[], void>({
      query: () => '/admin/exercise',
      providesTags: (result = []) =>
        [
          ...result.map(({id}) => ({type: 'Exercise' as const, id})),
          {type: 'Exercise', id: 'LIST'},
        ],
    }),
    adminGetExercise: builder.query<Exercise, string>({
      query: exerciseId => `/admin/exercise/${exerciseId}`,
      providesTags: (result, error, id) => [{type: 'Exercise', id}],
    }),
    adminAddExercise: builder.mutation<Exercise, NewExercise>({
      query: newExercise => ({
        url: '/admin/exercise', method: 'POST', body: newExercise,
      }),
      invalidatesTags: [{type: 'Exercise', id: 'LIST'}],
    }),
    adminUpdateExercise: builder.mutation<Exercise, {
      exerciseUpdate: UpdateExercise; exerciseId: string;
    }>({
      query: ({exerciseUpdate, exerciseId}) => ({
        url: `/admin/exercise/${exerciseId}`,
        method: 'PUT',
        body: exerciseUpdate,
      }),
      invalidatesTags: (result, error, {exerciseId}) =>
        [{type: 'Exercise', id: exerciseId}],
    }),
    adminDeleteExercise: builder
      .mutation<string, {exerciseId: string}>({
      query: ({exerciseId}) => ({
        url: `/admin/exercise/${exerciseId}`,
        method: 'DELETE',
        responseHandler: 'text',
      }),
      invalidatesTags: (result, error, {exerciseId}) =>
        [{type: 'Exercise', id: exerciseId}],
    }),
    adminGetDeployments: builder.query<Deployment[], string>({
      query: exerciseId => `/admin/exercise/${exerciseId}/deployment`,
      providesTags: (result = []) =>
        [
          ...result.map(({id}) => ({type: 'Deployment' as const, id})),
          {type: 'Deployment', id: 'LIST'},
        ],
    }),
    adminAddDeployment: builder
      .mutation<Deployment,
    {
      newDeployment: NewDeployment; exerciseId: string;
    }>({
      query: ({newDeployment, exerciseId}) => ({
        url: `/admin/exercise/${exerciseId}/deployment`,
        method: 'POST',
        body: newDeployment,
      }),
      invalidatesTags: [{type: 'Deployment', id: 'LIST'}],
    }),
    adminGetDeployment: builder.query<Deployment,
    {exerciseId: string; deploymentId: string}>({
      query: ({exerciseId, deploymentId}) =>
        `/admin/exercise/${exerciseId}/deployment/${deploymentId}`,
    }),
    adminDeleteDeployment: builder
      .mutation<string, {exerciseId: string; deploymentId: string}>({
      query: ({exerciseId, deploymentId}) => ({
        url: `/admin/exercise/${exerciseId}/deployment/${deploymentId}/users`,
        method: 'DELETE',
        responseHandler: 'text',
      }),
      invalidatesTags: (result, error, {deploymentId}) =>
        [{type: 'Deployment', id: deploymentId}],
    }),
    adminGetDeploymentElements: builder
      .query<DeploymentElement[], {exerciseId: string; deploymentId: string}>({
      query: ({exerciseId, deploymentId}) =>
        `/admin/exercise/${exerciseId}/deployment/${deploymentId}/deployment_element`,
    }),
    adminGetDeploymentGroups: builder.query<Deployers, void>({
      query: () => '/admin/deployer',
    }),
    adminSendEmail: builder
      .mutation <EmailForm, {email: EmailForm; exerciseId: string} >({
      query: ({email, exerciseId}) => ({
        url: `/admin/exercise/${exerciseId}/email`,
        method: 'POST',
        body: email,
      }),
    }),
    adminGetEmailForm: builder.query <string, string>({
      query: exerciseId => `/admin/exercise/${exerciseId}/email`,
    }),
    adminGetDeploymentScores: builder.query<Score[],
    {
      exerciseId: string;
      deploymentId: string;
    }>({
      query: ({exerciseId, deploymentId}) =>
        `/admin/exercise/${exerciseId}/deployment/${deploymentId}/score`,
    }),
    adminGetDeploymentScenario: builder.query<Scenario | undefined,
    {
      exerciseId: string;
      deploymentId: string;
    }>({
      query: ({exerciseId, deploymentId}) =>
        `/admin/exercise/${exerciseId}/deployment/${deploymentId}/scenario`,
    }),
    participantGetExercises: builder.query<ParticipantExercise[], void>({
      query: () => '/participant/exercise',
      providesTags: (result = []) =>
        [
          ...result.map(({id}) => ({type: 'Exercise' as const, id})),
          {type: 'Exercise', id: 'LIST'},
        ],
    }),
    participantGetExercise: builder.query<ParticipantExercise, string>({
      query: exerciseId => `/participant/exercise/${exerciseId}`,
      providesTags: (result, error, id) => [{type: 'Exercise', id}],
    }),
    participantGetDeployments: builder.query<ParticipantDeployment[], string>({
      query: exerciseId =>
        `/participant/exercise/${exerciseId}/deployment`,
    }),
    participantGetDeployment: builder.query<ParticipantDeployment,
    {
      exerciseId: string;
      deploymentId: string;
    }>({
      query: ({exerciseId, deploymentId}) =>
        `/participant/exercise/${exerciseId}/deployment/${deploymentId}`,
    }),
    participantGetDeploymentUsers: builder.query<AdUser[],
    {
      deploymentId: string;
      exerciseId: string;
    }>({
      query: ({deploymentId, exerciseId}) =>
        `/participant/exercise/${exerciseId}/deployment/${deploymentId}/users`,
    }),
    participantGetDeploymentScores: builder.query<Score[],
    {
      exerciseId: string;
      deploymentId: string;
    }>({
      query: ({exerciseId, deploymentId}) =>
        `/participant/exercise/${exerciseId}/deployment/${deploymentId}/score`,
    }),
    participantGetDeploymentScenario: builder.query<Scenario | undefined,
    {
      exerciseId: string;
      deploymentId: string;
      entitySelector: string;
    }>({
      query({exerciseId, deploymentId, entitySelector}) {
        return `/participant/exercise/${exerciseId}/deployment/${deploymentId}/scenario/${entitySelector}`;
      },
    }),
    participantGetDeploymentParticipants: builder.query<Participant[] | undefined,
    {
      exerciseId: string;
      deploymentId: string;
    }>({
      query({exerciseId, deploymentId}) {
        return `/participant/exercise/${exerciseId}/deployment/${deploymentId}/participant`;
      },
    }),
  }),
});

export const {
  useAdminGetGroupsQuery,
  useAdminGetGroupUsersQuery,
  useAdminGetDeploymentUsersQuery,
  useAdminGetExerciseQuery,
  useAdminGetExercisesQuery,
  useAdminAddExerciseMutation,
  useAdminDeleteExerciseMutation,
  useAdminUpdateExerciseMutation,
  useAdminGetDeploymentsQuery,
  useAdminAddDeploymentMutation,
  useAdminDeleteDeploymentMutation,
  useAdminGetDeploymentElementsQuery,
  useAdminGetDeploymentQuery,
  useAdminGetDeploymentScoresQuery,
  useAdminGetDeploymentGroupsQuery,
  useAdminSendEmailMutation,
  useAdminGetEmailFormQuery,
  useAdminGetDeploymentScenarioQuery,
  useParticipantGetExercisesQuery,
  useParticipantGetExerciseQuery,
  useParticipantGetDeploymentsQuery,
  useParticipantGetDeploymentQuery,
  useParticipantGetDeploymentUsersQuery,
  useParticipantGetDeploymentScoresQuery,
  useParticipantGetDeploymentScenarioQuery,
  useParticipantGetDeploymentParticipantsQuery,
} = apiSlice;
