import {skipToken} from '@reduxjs/toolkit/dist/query';
import React from 'react';
import {useParams} from 'react-router-dom';
import {type DeploymentEvent} from 'src/models/exercise';
import {type DeploymentDetailRouteParameters} from 'src/models/routes';
import {useAdminGetEventInfoQuery} from 'src/slices/apiSlice';
import EventIframe from 'src/components/Deployment/Event/EventIframe';

const EventInfo = ({event_name, event}:
{event_name: string;
  event: DeploymentEvent;
}) => {
  const {exerciseId, deploymentId} = useParams<DeploymentDetailRouteParameters>();
  const eventInfoDataChecksum = event?.eventInfoDataChecksum;
  const {data: eventInfo} = useAdminGetEventInfoQuery(
    exerciseId && deploymentId && eventInfoDataChecksum
      ? {exerciseId, deploymentId, eventInfoDataChecksum} : skipToken);

  if (!eventInfoDataChecksum) {
    return null;
  }

  return (
    <div key={event.id} className='p-2'>
      <details className='p-2 border-2 border-slate-300 shadow-md '>
        <summary className='font-bold text-xl'>
          {event_name ?? event.name}
        </summary>
        <div className='mt-2 text-sm'>
          <div>
            <EventIframe eventInfo={eventInfo}/>
          </div>
        </div>
      </details>
    </div>
  );
};

export default EventInfo;
