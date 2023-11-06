import React from 'react';
import {H2} from '@blueprintjs/core';
import {type Banner} from 'src/models/exercise';
import {useKeycloak} from '@react-keycloak/web';
import {parseBannerForParticipant} from 'src/utils/banner';
import {
  useParticipantGetDeploymentQuery,
  useParticipantGetExerciseQuery,
} from 'src/slices/apiSlice';
import MarkdownFrame from 'src/components/MarkdownFrame';

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

  const parsedBanner = parseBannerForParticipant(
    existingBanner,
    exercise.name,
    deployment.name,
    keycloak.tokenParsed.preferred_username as string,
  );

  return (
    <div>
      <H2>{parsedBanner.name}</H2>
      <MarkdownFrame content={parsedBanner.content}/>
    </div>
  );
};

export default ParticipantDashBoard;
