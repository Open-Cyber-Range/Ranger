import React from 'react';
import {H2} from '@blueprintjs/core';
import {type Banner} from 'src/models/exercise';

const ParticipantDashBoard = ({existingBanner}:
{existingBanner: Banner | undefined},
) => (
  <>
    <H2>{existingBanner?.name}</H2>
    <p>{existingBanner?.content}</p>
  </>
);

export default ParticipantDashBoard;
