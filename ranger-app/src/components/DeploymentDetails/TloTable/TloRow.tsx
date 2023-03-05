import React from 'react';
import {useTranslation} from 'react-i18next';
import type {TrainingLearningObjective} from 'src/models/tlo';
import TloDetails from './TloDetails';

const TloRow = ({exerciseId, deploymentId, tloName, tloMap}:
{exerciseId: string;
  deploymentId: string;
  tloName: string;
  tloMap: Record<string, TrainingLearningObjective>;
}) => {
  const {t} = useTranslation();

  if (tloMap[tloName]) {
    return (
      <tr key={tloName}>
        <td>
          <div>{tloName}</div>
          <div>{tloMap[tloName].description}</div>
        </td>
        <TloDetails
          key={tloName}
          exerciseId={exerciseId}
          deploymentId={deploymentId}
          tloName={tloName}
          tloMap={tloMap}
        />
      </tr>
    );
  }

  return (
    <tr>
      {t('error.none')}
    </tr>
  );
};

export default TloRow;
