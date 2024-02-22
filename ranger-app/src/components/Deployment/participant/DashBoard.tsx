import React from 'react';
import {H2, H4, Icon} from '@blueprintjs/core';
import {type Banner} from 'src/models/exercise';
import {useKeycloak} from '@react-keycloak/web';
import {parseBannerForParticipant} from 'src/utils/banner';
import {
  useParticipantGetDeploymentQuery,
  useParticipantGetExerciseQuery,
} from 'src/slices/apiSlice';
import {useTranslation} from 'react-i18next';
import {formatStringToDateTime} from 'src/utils';
import DOMPurify from 'dompurify';
import ContentIFrame from 'src/components/ContentIFrame';

const ParticipantDashBoard = ({exerciseId, deploymentId, existingBanner}:
{exerciseId: string; deploymentId: string; existingBanner: Banner | undefined},
) => {
  const {t} = useTranslation();
  const {keycloak} = useKeycloak();
  const {data: exercise} = useParticipantGetExerciseQuery(exerciseId);
  const {data: deployment} = useParticipantGetDeploymentQuery(
    {exerciseId, deploymentId},
  );

  if (!deployment) {
    return (
      <div className='flex flex-col h-full min-h-screen'>
        <H2>{existingBanner?.name}</H2>
        <p>{existingBanner?.content}</p>
      </div>
    );
  }

  if (!exercise || !existingBanner || !keycloak.tokenParsed) {
    return (
      <div className='flex flex-col h-full min-h-screen'>
        <div className='pt-2 pb-4'>
          <div className='flex'>
            <Icon icon='time' size={22}/>
            <H4 className='font-bold pl-2'>{t('deployments.startTime')} </H4>
          </div>
          {formatStringToDateTime(deployment.start)}
        </div>
        <div className='pt-2 pb-4'>
          <div className='flex'>
            <Icon icon='time' size={22}/>
            <H4 className='font-bold pl-2'>{t('deployments.endTime')} </H4>
          </div>
          {formatStringToDateTime(deployment.end)}
        </div>
      </div>
    );
  }

  const encoded = new Uint8Array(existingBanner.content);
  let htmlString = new TextDecoder().decode(encoded);
  htmlString = DOMPurify.sanitize(htmlString);

  const parsedContent = parseBannerForParticipant(
    htmlString,
    exercise.name,
    deployment.name,
    keycloak.tokenParsed.preferred_username as string,
  );

  const parsedUint8Array = new TextEncoder().encode(parsedContent.content);
  return (
    <div className='flex flex-col h-full min-h-screen'>
      <div className='pt-2 pb-4'>
        <div className='flex'>
          <Icon icon='time' size={22}/>
          <H4 className='font-bold pl-2'>{t('deployments.startTime')} </H4>
        </div>
        {formatStringToDateTime(deployment.start)}
      </div>
      <div className='pt-2 pb-4'>
        <div className='flex'>
          <Icon icon='time' size={22}/>
          <H4 className='font-bold pl-2'>{t('deployments.endTime')} </H4>
        </div>
        {formatStringToDateTime(deployment.end)}
      </div>
      <ContentIFrame content={parsedUint8Array}/>
    </div>
  );
};

export default ParticipantDashBoard;
