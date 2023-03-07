import type React from 'react';
import PageHolder from 'src/components/PageHolder';
import {useTranslation} from 'react-i18next';
import type {Deployment} from 'src/models/deployment';
import styled from 'styled-components';
import ScoreTags from 'src/components/Deployment/ScoreTags/ScoreTags';

const Wrapper = styled.div`
  display: flex;
  flex-direction: column;
  text-align: center;
`;

const FallbackTextWrapper = styled.div`
  display: flex;
  justify-content: center;
  align-self: center;
  margin-top: 5rem;
  margin-bottom: 1rem;
  color: #a6a3a3;
`;

const DataCellWrapper = styled.td`
  display: flex;
  flex-direction: row;
  justify-content: space-between;
`;

const ScoresPanel = ({deployments}:
{deployments: Deployment[] | undefined;
}) => {
  const {t} = useTranslation();

  if (deployments) {
    return (
      <PageHolder>
        <Wrapper>
          <table className='
              bp4-html-table
              bp4-compact
              bp4-html-table-striped
              '
          >
            <tbody>
              {deployments.map(deployment => (
                <tr key={deployment.id}>
                  <DataCellWrapper>
                    <h2>{deployment.name}</h2>
                    <ScoreTags
                      exerciseId={deployment.exerciseId}
                      deploymentId={deployment.id}/>
                  </DataCellWrapper>
                </tr>
              ))}
            </tbody>
          </table>
        </Wrapper>
      </PageHolder>
    );
  }

  return (
    <FallbackTextWrapper>
      {t('exercises.noDeployments')}
    </FallbackTextWrapper>
  );
};

export default ScoresPanel;
