import {createApi, fetchBaseQuery} from '@reduxjs/toolkit/query/react';
import {BASE_URL} from 'src/constants';
import type {
  Deployers,
  Deployment,
  DeploymentElement,
  NewDeployment,
  ParticipantDeployment,
} from 'src/models/deployment';
import type {Participant, NewParticipant} from 'src/models/participant';
import {
  type Banner,
  type ParticipantExercise,
  type Exercise,
  type NewExercise,
  type UpdateExercise,
  type DeploymentEvent,
  type EventInfo,
} from 'src/models/exercise';
import type {EmailForm, Email} from 'src/models/email';
import {type AdGroup, type AdUser} from 'src/models/groups';
import {type Scenario} from 'src/models/scenario';
import {type Score} from 'src/models/score';
import {type RootState} from 'src/store';
import {
  type UpdateManualMetric,
  type ManualMetric,
  type FetchArtifact,
  type NewManualMetric,
} from 'src/models/manualMetric';

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
  tagTypes: ['Deployment',
    'Exercise',
    'Score',
    'Scenario',
    'Participant',
    'ManualMetric',
    'Email'],
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
    adminGetBanner: builder.query<Banner, string>({
      query: exerciseId => `/admin/exercise/${exerciseId}/banner`,
      providesTags: (result, error, id) => [{type: 'Exercise', id}],
    }),
    adminAddBanner: builder
      .mutation<Banner,
    {newBanner: Banner; exerciseId: string}>({
      query: ({newBanner, exerciseId}) => ({
        url: `/admin/exercise/${exerciseId}/banner`,
        method: 'POST',
        body: newBanner,
      }),
      invalidatesTags: [{type: 'Exercise', id: 'LIST'}],
    }),
    adminDeleteBanner: builder
      .mutation<string, {exerciseId: string}>({
      query: ({exerciseId}) => ({
        url: `/admin/exercise/${exerciseId}/banner`,
        method: 'DELETE',
        responseHandler: 'text',
      }),
      invalidatesTags: (result, error, {exerciseId}) =>
        [{type: 'Exercise', id: exerciseId}],
    }),
    adminUpdateBanner: builder
      .mutation<Banner,
    {updatedBanner: Banner; exerciseId: string}>({
      query: ({updatedBanner, exerciseId}) => ({
        url: `/admin/exercise/${exerciseId}/banner`,
        method: 'PUT',
        body: updatedBanner,
      }),
      invalidatesTags: (result, error, {exerciseId}) =>
        [{type: 'Exercise', id: exerciseId}],
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
        url: `/admin/exercise/${exerciseId}/deployment/${deploymentId}`,
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
    adminGetDefaultDeploymentGroup: builder.query<string, void>({
      query: () => '/admin/deployer/default',
    }),
    adminGetEmails: builder.query<Email[], string>({
      query: exerciseId => `/admin/exercise/${exerciseId}/email`,
      providesTags: (result = []) =>
        [
          ...result.map(({id}) => ({type: 'Email' as const, id})),
          {type: 'Email', id: 'LIST'},
        ],
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
      query: exerciseId => `/admin/exercise/${exerciseId}/email-form`,
    }),
    adminUploadFile: builder.mutation<string, FormData>({
      query: formData => ({
        url: '/upload',
        method: 'POST',
        body: formData,
      }),
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
    adminAddParticipant: builder
      .mutation <Participant, {
      exerciseId: string;
      deploymentId: string;
      newParticipant: NewParticipant;
    } >({
      query: ({deploymentId, exerciseId, newParticipant}) => ({
        url: `/admin/exercise/${exerciseId}/deployment/${deploymentId}/participant`,
        method: 'POST',
        body: newParticipant,
      }),
      invalidatesTags: [{type: 'Participant', id: 'LIST'}],
    }),
    adminDeleteParticipant: builder
      .mutation<Participant, {
      exerciseId: string;
      deploymentId: string;
      participantId: string;
    }>({
      query: ({exerciseId, deploymentId, participantId}) => ({
        url: `/admin/exercise/${
          exerciseId}/deployment/${deploymentId}/participant/${participantId}`,
        method: 'DELETE',
        responseHandler: 'text',
      }),
      invalidatesTags: [{type: 'Participant', id: 'LIST'}],
    }),
    adminGetDeploymentParticipants: builder.query<Participant[] | undefined,
    {
      exerciseId: string;
      deploymentId: string;
    }>({
      query({exerciseId, deploymentId}) {
        return `/admin/exercise/${exerciseId}/deployment/${deploymentId}/participant`;
      },
      providesTags: (result = []) =>
        [
          ...result.map(({id}) => ({type: 'Exercise' as const, id})),
          {type: 'Participant', id: 'LIST'},
        ],
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
    participantGetDeploymentScores: builder.query<Score[], //
    {
      exerciseId: string;
      deploymentId: string;
      entitySelector: string;
    }>({
      query: ({exerciseId, deploymentId, entitySelector}) =>
        // eslint-disable-next-line max-len
        `/participant/exercise/${exerciseId}/deployment/${deploymentId}/entity/${entitySelector}/score`,
    }),
    participantGetDeploymentScenario: builder.query<Scenario | undefined,
    {
      exerciseId: string;
      deploymentId: string;
      entitySelector: string;
    }>({
      query({exerciseId, deploymentId, entitySelector}) {
        return `/participant/exercise/${
          exerciseId}/deployment/${deploymentId}/scenario/${entitySelector}`;
      },
    }),
    participantGetOwnParticipants: builder.query<Participant[] | undefined,
    {
      exerciseId: string;
      deploymentId: string;
    }>({
      query({exerciseId, deploymentId}) {
        return `/participant/exercise/${exerciseId}/deployment/${deploymentId}/participant`;
      },
    }),
    participantGetTriggeredEvents: builder.query<DeploymentEvent[] | undefined,
    {
      exerciseId: string;
      deploymentId: string;
      entitySelector: string;
    }>({
      query({exerciseId, deploymentId, entitySelector}) {
        return `/participant/exercise/${
          exerciseId}/deployment/${deploymentId}/entity/${entitySelector}/event`;
      },
    }),
    participantGetEventInfo: builder.query<EventInfo | undefined,
    {
      exerciseId: string;
      deploymentId: string;
      entitySelector: string;
      eventInfoDataChecksum: string;
    }>({
      query({exerciseId, deploymentId, entitySelector, eventInfoDataChecksum}) {
        return `/participant/exercise/${
          // eslint-disable-next-line max-len
          exerciseId}/deployment/${deploymentId}/entity/${entitySelector}/event/${eventInfoDataChecksum}`;
      },
    }),
    participantGetBanner: builder.query<Banner, string>({
      query: exerciseId =>
        `/participant/exercise/${exerciseId}/banner`,
    }),
    adminGetManualMetrics: builder.query<ManualMetric[] | undefined,
    {
      exerciseId: string;
      deploymentId: string;
    }>({
      query({exerciseId, deploymentId}) {
        return `/admin/exercise/${exerciseId}/deployment/${deploymentId}/metric`;
      },
      providesTags: (result = []) =>
        [
          ...result.map(({id}) => ({type: 'Exercise' as const, id})),
          {type: 'ManualMetric', id: 'LIST'},
        ],
    }),
    adminGetMetric: builder.query<ManualMetric | undefined,
    {
      exerciseId: string;
      deploymentId: string;
      metricId: string;
    }>({
      query: ({exerciseId, deploymentId, metricId}) =>
        `/admin/exercise/${exerciseId}/deployment/${deploymentId}/metric/${metricId}`,
    }),
    adminUpdateMetric: builder.mutation<ManualMetric, {
      exerciseId: string;
      deploymentId: string;
      metricId: string;
      manualMetricUpdate: UpdateManualMetric;
    }>({
      query: ({manualMetricUpdate, exerciseId, deploymentId, metricId}) => ({
        url: `/admin/exercise/${exerciseId}/deployment/${deploymentId}/metric/${metricId}`,
        method: 'PUT',
        body: manualMetricUpdate,
      }),
    }),
    adminDeleteMetric: builder
      .mutation<string, {
      deploymentId: string;
      exerciseId: string;
      metricId: string;
    }>({
      query: ({exerciseId, deploymentId, metricId}) => ({
        url: `/admin/exercise/${exerciseId}/deployment/${deploymentId}/metric/${metricId}`,
        method: 'DELETE',
        responseHandler: 'text',
      }),
      invalidatesTags: ['ManualMetric'],
    }),
    adminGetManualMetricArtifact: builder.query<FetchArtifact, {
      exerciseId: string;
      deploymentId: string;
      metricId: string;
    }>({
      query: ({exerciseId, deploymentId, metricId}) => ({
        url: `/admin/exercise/${exerciseId}/deployment/${deploymentId}/metric/${metricId}/download`,
        method: 'GET',
        async responseHandler(response) {
          const filename = response.headers.get('Content-Disposition')?.split('filename=')[1];
          const blob = await response.blob();
          const url = URL.createObjectURL(blob);
          return {filename, url};
        },

        cache: 'no-store',
      }),
    }),
    participantGetMetric: builder.query<ManualMetric | undefined,
    {
      exerciseId: string;
      deploymentId: string;
      metricId: string;
      entitySelector: string;
    }>({
      query: ({exerciseId, deploymentId, metricId, entitySelector}) =>
        // eslint-disable-next-line max-len
        `/participant/exercise/${exerciseId}/deployment/${deploymentId}/entity/${entitySelector}/metric/${metricId}`,
    }),
    participantGetMetrics: builder.query<ManualMetric[],
    {
      exerciseId: string;
      deploymentId: string;
      entitySelector: string;
    }>({
      query({exerciseId, deploymentId, entitySelector}) {
        // eslint-disable-next-line max-len
        return `/participant/exercise/${exerciseId}/deployment/${deploymentId}/entity/${entitySelector}/metric`;
      },
      providesTags: ['ManualMetric'],
    }),
    participantUpdateMetric: builder.mutation<ManualMetric, {
      exerciseId: string;
      deploymentId: string;
      metricId: string;
      manualMetricUpdate: UpdateManualMetric;
      entitySelector: string;
    }>({
      query: ({manualMetricUpdate, exerciseId, deploymentId, metricId, entitySelector}) => ({
        // eslint-disable-next-line max-len
        url: `/participant/exercise/${exerciseId}/deployment/${deploymentId}/entity/${entitySelector}/metric/${metricId}`,
        method: 'PUT',
        body: manualMetricUpdate,
      }),
      invalidatesTags: ['ManualMetric'],
    }),
    participantAddMetric: builder.mutation<string, {
      exerciseId: string;
      deploymentId: string;
      newManualMetric: NewManualMetric;
      entitySelector: string;
    }>({
      query: ({newManualMetric, exerciseId, deploymentId, entitySelector}) => ({
        // eslint-disable-next-line max-len
        url: `/participant/exercise/${exerciseId}/deployment/${deploymentId}/entity/${entitySelector}/metric`,
        method: 'POST',
        body: newManualMetric,
      }),
      invalidatesTags: ['ManualMetric'],
    }),
    participantUploadMetricArtifact: builder.mutation<string, {
      exerciseId: string;
      deploymentId: string;
      metricId: string;
      artifactFile: File;
      entitySelector: string;
    }>({
      query({exerciseId, deploymentId, metricId, artifactFile, entitySelector}) {
        const formData = new FormData();
        formData.append('artifact', artifactFile, artifactFile.name);

        return {
        // eslint-disable-next-line max-len
          url: `/participant/exercise/${exerciseId}/deployment/${deploymentId}/entity/${entitySelector}/metric/${metricId}/upload`,
          method: 'POST',
          body: formData,
          formData: true,
        };
      },
      invalidatesTags: ['ManualMetric'],
    }),
    participantGetNodeDeploymentElements: builder
      .query<DeploymentElement[],
    {exerciseId: string; deploymentId: string; entitySelector: string}>({
      query: ({exerciseId, deploymentId, entitySelector}) =>
        // eslint-disable-next-line max-len
        `/participant/exercise/${exerciseId}/deployment/${deploymentId}/entity/${entitySelector}/deployment_element`,
    }),
  }),
});

export const {
  useAdminGetGroupsQuery,
  useAdminGetGroupUsersQuery,
  useAdminGetDeploymentUsersQuery,
  useAdminGetBannerQuery,
  useAdminAddBannerMutation,
  useAdminDeleteBannerMutation,
  useAdminUpdateBannerMutation,
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
  useAdminGetDefaultDeploymentGroupQuery,
  useAdminGetEmailsQuery,
  useAdminSendEmailMutation,
  useAdminGetEmailFormQuery,
  useAdminUploadFileMutation,
  useAdminGetDeploymentScenarioQuery,
  useAdminAddParticipantMutation,
  useAdminDeleteParticipantMutation,
  useAdminGetDeploymentParticipantsQuery,
  useAdminGetManualMetricsQuery,
  useAdminGetMetricQuery,
  useAdminUpdateMetricMutation,
  useAdminDeleteMetricMutation,
  useLazyAdminGetManualMetricArtifactQuery,
  useParticipantGetExercisesQuery,
  useParticipantGetExerciseQuery,
  useParticipantGetDeploymentsQuery,
  useParticipantGetDeploymentQuery,
  useParticipantGetDeploymentUsersQuery,
  useParticipantGetDeploymentScoresQuery,
  useParticipantGetDeploymentScenarioQuery,
  useParticipantGetOwnParticipantsQuery,
  useParticipantGetTriggeredEventsQuery,
  useParticipantGetEventInfoQuery,
  useParticipantGetBannerQuery,
  useParticipantGetMetricQuery,
  useParticipantGetMetricsQuery,
  useParticipantUpdateMetricMutation,
  useParticipantAddMetricMutation,
  useParticipantUploadMetricArtifactMutation,
  useParticipantGetNodeDeploymentElementsQuery,
} = apiSlice;
