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

  if (tloMap) {
    const sortedTloNames = Object.keys(tloMap).sort((a, b) => (a > b ? 1 : -1));

    return (
      <Wrapper>
        <table className='
          bp4-html-table
          bp4-html-table-striped'
        >
          <thead>
            <tr>
              <th>{t('tloTable.headers.tlo')}</th>
              <th>{t('tloTable.headers.evaluation')}</th>
              <th>{t('tloTable.headers.metric')}</th>
            </tr>
          </thead>
          <tbody>
            {sortedTloNames.map(tloName => (
              <TloRow
                key={tloName}
                exerciseId={exerciseId}
                deploymentId={deploymentId}
                tloName={tloName}
                tloMap={tloMap}/>
            ))}
          </tbody>
        </table>
      </Wrapper>
    );
  }

  return null;
};

export default TloTable;
