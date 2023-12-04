import {skipToken} from '@reduxjs/toolkit/dist/query';
import React, {useEffect, useState} from 'react';
import {useTranslation} from 'react-i18next';
import {useSelector} from 'react-redux';
import {useParams} from 'react-router-dom';
import {type DeploymentEvent} from 'src/models/exercise';
import {type DeploymentDetailRouteParameters} from 'src/models/routes';
import {useParticipantGetEventInfoQuery} from 'src/slices/apiSlice';
import {selectedEntity} from 'src/slices/userSlice';
import DOMPurify from 'dompurify';

async function fetchCSS(cssURL: string) {
  const response = await fetch(cssURL);
  const cssText = await response.text();
  return cssText;
}

const EventInfo = ({event_name, event}:
{event_name: string | undefined; event: DeploymentEvent ;
}) => {
  const {t} = useTranslation();
  const {exerciseId, deploymentId} = useParams<DeploymentDetailRouteParameters>();
  const entitySelector = useSelector(selectedEntity);
  const eventInfoDataChecksum = event?.eventInfoDataChecksum;
  const {data: eventInfo} = useParticipantGetEventInfoQuery(
    exerciseId && deploymentId && entitySelector && eventInfoDataChecksum
      ? {exerciseId, deploymentId, entitySelector, eventInfoDataChecksum} : skipToken);

  const [url, setUrl] = useState<string | undefined>(undefined);
  const cssLink = '/gfm.min.css';

  useEffect(() => {
    const encoded = eventInfo?.content ? new Uint8Array(eventInfo.content) : new Uint8Array();
    let htmlString = new TextDecoder().decode(encoded);
    htmlString = DOMPurify.sanitize(htmlString);

    fetchCSS(cssLink).then(cssStyles => {
      const htmlWithCSS = `
        <!DOCTYPE html>
        <html>
          <head>
            <meta charset="UTF-8">
            <style>
              ${cssStyles}
            </style>
          </head>
          <body>${htmlString}</body>
        </html>
      `;

      const blob = new Blob([htmlWithCSS], {type: 'text/html'});
      const blobUrl = URL.createObjectURL(blob);

      setUrl(blobUrl);
    }).catch(_ => {
      const htmlWithoutCSS = `
      <!DOCTYPE html>
      <html>
        <head>
          <meta charset="UTF-8">
        </head>
        <body>${htmlString}</body>
      </html>
    `;

      const blob = new Blob([htmlWithoutCSS], {type: 'text/html'});
      const blobUrl = URL.createObjectURL(blob);

      setUrl(blobUrl);
    });
  }, [eventInfo, cssLink]);

  return url ? (
    <div key={event.id} className='p-2'>
      <details className='p-2 border-2 border-slate-300 shadow-md '>
        <summary className='font-bold text-xl'>
          {event_name ?? event.name}
        </summary>
        <div className='mt-2 text-sm'>
          <div>
            <iframe
              className='w-full h-screen'
              src={url}
              sandbox='allow-same-origin'
              title='HTML content'/>
          </div>
          {event.description ?? t('participant.exercise.events.noDescription')}
          <div className='text-slate-600 italic'>
            <br/>
            {t('participant.exercise.events.triggeredAt')}{': '}
            {new Date(event.triggeredAt).toLocaleString()}
          </div>
        </div>
      </details>
    </div>
  ) : null;
};

export default EventInfo;
