import React from 'react';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {useTranslation} from 'react-i18next';
import {
  useParticipantGetDeploymentScenarioQuery,
  useParticipantGetTriggeredEventsQuery,
} from 'src/slices/apiSlice';
import {useSelector} from 'react-redux';
import {selectedEntity} from 'src/slices/userSlice';

const Events = ({
  exerciseId, deploymentId}:
{exerciseId: string; deploymentId: string;
}) => {
  const {t} = useTranslation();
  const entitySelector = useSelector(selectedEntity);
  const scenarioQueryArguments = exerciseId && deploymentId && entitySelector
    ? {exerciseId, deploymentId, entitySelector} : skipToken;
  const {data: scenario} = useParticipantGetDeploymentScenarioQuery(scenarioQueryArguments);
  const {data: triggeredDeploymentEvents}
  = useParticipantGetTriggeredEventsQuery(scenarioQueryArguments);
  const scenarioEvents = scenario?.events;

  if (scenarioEvents && triggeredDeploymentEvents && triggeredDeploymentEvents.length > 0) {
    return (
      <>
        {
          triggeredDeploymentEvents.map(event => (
            <div key={event.id} className='p-2'>
              <details className='p-2 border-2 border-slate-300 shadow-md '>
                <summary className='font-bold text-xl'>
                  {scenarioEvents[event.name].name ?? event.name}
                </summary>
                <div className='mt-2 text-sm'>
                  {event.description
                 ?? t('participant.exercise.events.noDescription')}
                  <div className='text-slate-600 italic'>
                    <br/>
                    {t('participant.exercise.events.triggeredAt')}{': '}
                    {new Date(event.triggeredAt).toLocaleString()}
                  </div>
                </div>
              </details>
            </div>
          ))
        }
      </>
    );
  }

  if (triggeredDeploymentEvents && triggeredDeploymentEvents.length === 0) {
    return (
      <div className='
      flex justify-center align-center m-2 mt-auto mb-4 text-gray-400'
      >
        {t('participant.exercise.events.noTriggeredEvents')}
      </div>
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
