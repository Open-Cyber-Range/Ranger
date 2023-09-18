import React from 'react';
import {useTranslation} from 'react-i18next';
import {type DeploymentEvent} from 'src/models/exercise';
import {type Event} from 'src/models/scenario';

const Events = ({scenarioEvents, deploymentEvents}:
{scenarioEvents: Record<string, Event> | undefined; deploymentEvents: DeploymentEvent[] | undefined;
}) => {
  const {t} = useTranslation();

  if (scenarioEvents && deploymentEvents && deploymentEvents.length > 0) {
    return (
      <>
        {
          deploymentEvents.map(event => (
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

  if (deploymentEvents && deploymentEvents.length === 0) {
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
