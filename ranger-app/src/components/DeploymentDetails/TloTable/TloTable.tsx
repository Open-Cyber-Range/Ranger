import React from 'react';
import {useTranslation} from 'react-i18next';
import type {TrainingLearningObjective} from 'src/models/tlo';
import styled from 'styled-components';
import TloRow from './TloRow';

const Wrapper = styled.div`
  display: flex;
  flex-direction: column;
  text-align: center;
  margin-top: 2rem;
`;

const TloTable = ({exerciseId, deploymentId, tloMap}:
{exerciseId: string;
  deploymentId: string;
  tloMap: Record<string, TrainingLearningObjective> | undefined;
}) => {
  const {t} = useTranslation();

  if (!tloMap) {
    return null;
  }

  return (
    <Wrapper>
      <table className='
    bp4-html-table
    bp4-html-table-striped'
      >
        <thead>
          <tr>
            <th>{t('sdl.tlo')}</th>
            <th>{t('sdl.evaluation')}</th>
            <th>{t('sdl.metrics')} - {t('common.currentScore')}</th>
          </tr>
        </thead>
        <tbody>
          {(() => {
            const rows = [];
            for (const tloName in tloMap) {
              if (tloMap[tloName]) {
                rows.push(
                  <TloRow
                    key={tloName}
                    exerciseId={exerciseId}
                    deploymentId={deploymentId}
                    tloName={tloName}
                    tloMap={tloMap}/>,
                );
              }
            }

            return rows;
          })()}
        </tbody>
      </table>
    </Wrapper>
  );
};

export default TloTable;
