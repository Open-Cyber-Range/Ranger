import React, {useEffect, useState} from 'react';
import {type EventInfo} from 'src/models/exercise';
import DOMPurify from 'dompurify';

async function fetchCSS(cssURL: string) {
  const response = await fetch(cssURL);
  return response.text();
}

const EventIframe = ({eventInfo}:
{
  eventInfo: EventInfo | undefined;
}) => {
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

  if (!eventInfo || !url) {
    return null;
  }

  return (
    <iframe
      className='w-full h-screen'
      src={url}
      sandbox='allow-same-origin'
      title='HTML content'/>
  );
};

export default EventIframe;
