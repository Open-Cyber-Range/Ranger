import React from 'react';
import {H2, H3} from '@blueprintjs/core';
import {type Banner} from 'src/models/exercise';

const ParticipantDashBoard = ({existingBanner}:
{existingBanner: Banner | undefined},
) => (
  <>
    <H2>{existingBanner?.name}</H2>
    <H3>{existingBanner?.content}</H3>
  </>
);

export default ParticipantDashBoard;
