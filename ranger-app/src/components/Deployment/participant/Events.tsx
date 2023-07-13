import React from 'react';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {useTranslation} from 'react-i18next';
import {useParticipantGetDeploymentScenarioQuery} from 'src/slices/apiSlice';

const Events = ({
  exerciseId, deploymentId}: {exerciseId: string; deploymentId: string;
}) => {
  const {t} = useTranslation();
  const queryArguments = exerciseId && deploymentId
    ? {exerciseId, deploymentId} : skipToken;
  const {data: scenario} = useParticipantGetDeploymentScenarioQuery(queryArguments);

  if (scenario?.events) {
    const events = Object.entries(scenario.events);

    return (
      events.map(([event_key, event]) => (
        <div key={event.name} className='p-2'>
          <details className='p-2 border-2 border-slate-300 shadow-md '>
            <summary className='font-bold text-xl'>
              {event.name ?? event_key}
            </summary>
            <div className='mt-2 text-sm'>
              {event.description
             ?? t('participant.exercise.events.noDescription')}
            </div>
          </details>
        </div>

      ))
    );
  }

  return (
    <div className='
    flex justify-center align-center m-2 mt-auto mb-4 text-gray-400'
    >
      {t('participant.exercise.events.noEvents')}
    </div>
  );
};

export default Events;
