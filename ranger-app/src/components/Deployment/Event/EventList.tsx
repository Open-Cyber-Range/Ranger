import {Callout, H3, H4} from '@blueprintjs/core';
import React from 'react';
import {useTranslation} from 'react-i18next';
import {type DeploymentEvent} from 'src/models/exercise';
import {type Event} from 'src/models/scenario';
import {DateTime} from 'luxon';
import {type DeploymentElement} from 'src/models/deployment';
import {sortByProperty} from 'sort-by-property';
import PageHolder from 'src/components/PageHolder';
import EventInfo from './EventInfo';
import ProgressBarWithTimer from './EventProgressBar';

const formatStringToDateTime = (date: string) => DateTime.fromISO(date, {zone: 'utc'})
  .setZone('local')
  .toFormat('dd LLL yyyy HH:mm:ss');

const ManagerEvents = ({scenarioEvents, deploymentEvents, deploymentElements}:
{
  scenarioEvents: Record<string, Event> | undefined;
  deploymentEvents: DeploymentEvent[] | undefined;
  deploymentElements: DeploymentElement[] | undefined;
}) => {
  const {t} = useTranslation();

  if (scenarioEvents && deploymentEvents && deploymentEvents.length > 0) {
    const groupedEventsByName = deploymentEvents.slice()
      .sort(sortByProperty('start', 'asc'))
      .reduce<Record<string, DeploymentEvent[]>>((accumulator, event) => {
      if (!accumulator[event.name]) {
        accumulator[event.name] = [];
      }

      accumulator[event.name].push(event);
      return accumulator;
    }
    , {});

    return (
      <PageHolder>
        {
          Object.entries(groupedEventsByName).map(([eventName, events]) => {
            const now = DateTime.utc();
            const end = DateTime.fromISO(events[0].end, {zone: 'UTC'});

            return (
              <div key={eventName} className='border-2 rounded-lg p-4 mb-4 '>
                <H3 className='text-2xl font-bold mb-4'>{eventName}</H3>
                <ProgressBarWithTimer event={events[0]}/>
                <div className='mb-6 text-base'>
                  <div className='mb-6'>
                    <p>
                      <span className='font-medium'>{t('deployments.events.startTime')} </span>
                      {formatStringToDateTime(events[0].start)}
                    </p>
                    <p>
                      <span className='font-medium'>{t('deployments.events.endTime')} </span>
                      {formatStringToDateTime(events[0].end)}
                    </p>
                    <p>
                      <span className='font-medium'>{t('deployments.events.description')} </span>
                      {events[0].description ?? t('deployments.events.noDescription')}
                    </p>
                  </div>
                  <EventInfo event_name={eventName} event={events[0]}/>
                </div>
                <H4 className='mt-6 text-xl font-semibold'>
                  {t('deployments.events.nodes')}
                </H4>

                {events.sort(sortByProperty('parentNodeId', 'desc')).map(event => (
                  <div key={event.id} className='border-2 rounded-lg p-2 mb-4'>
                    {event.hasTriggered && <span className='text-green-500 text-xl'>✔ </span>}

                    {now > end && !event.hasTriggered
                    && <span className='text-red-500 text-xl'>❌ </span>}

                    <span key={event.id} className='font-bold mr-2 text-lg'>
                      {deploymentElements?.find(element =>
                        element.handlerReference
                        === event.parentNodeId)?.scenarioReference ?? event.parentNodeId}
                    </span>
                    <span className='text-sm'>
                      {event.hasTriggered
                        ? t('deployments.events.triggeredAt',
                          {date: formatStringToDateTime(event.triggeredAt)})
                        : t('deployments.events.notTriggered')}
                    </span>
                  </div>
                ))}
              </div>
            );
          })
        }
      </PageHolder>
    );
  }

  return (
    <Callout title={t('deployments.events.noEvents') ?? ''}/>
  );
};

export default ManagerEvents;
