import React from 'react';
import {H2} from '@blueprintjs/core';
import {type Banner} from 'src/models/exercise';
import {useKeycloak} from '@react-keycloak/web';
import {parseBannerForParticipant} from 'src/utils/banner';
import {
  useParticipantGetDeploymentQuery,
  useParticipantGetExerciseQuery,
} from 'src/slices/apiSlice';
import DOMPurify from 'dompurify';
import ContentIFrame from 'src/components/ContentIFrame';

const ParticipantDashBoard = ({exerciseId, deploymentId, existingBanner}:
{exerciseId: string; deploymentId: string; existingBanner: Banner | undefined},
) => {
  const {keycloak} = useKeycloak();
  const {data: exercise} = useParticipantGetExerciseQuery(exerciseId);
  const {data: deployment} = useParticipantGetDeploymentQuery(
    {exerciseId, deploymentId},
  );

  if (!exercise || !deployment || !existingBanner || !keycloak.tokenParsed) {
    return (
      <>
        <H2>{existingBanner?.name}</H2>
        <p>{existingBanner?.content}</p>
      </>
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
      <ContentIFrame content={parsedUint8Array}/>
    </div>
  );
};

export default ParticipantDashBoard;
