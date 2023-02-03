
import React from 'react';
import {useParams} from 'react-router-dom';

import PageHolder from 'src/components/PageHolder';
import type {DeploymentDetailRouteParameters} from 'src/models/routes';
import {useGetTLOsQuery} from 'src/slices/apiSlice';
import styled from 'styled-components';
import {definedOrSkipToken} from 'src/utils';
import TloRow from 'src/components/DeploymentDetails/TloTable/TloRow';
import DeploymentDetailsGraph from 'src/components/DeploymentDetails/Graph';

const Wrapper = styled.div`
  display: flex;
  flex-direction: column;
  text-align: center;

  > div {
    margin-bottom: 2rem;
  }
`;

const DeploymentDetail = () => {
  const {exerciseId, deploymentId}
  = useParams<DeploymentDetailRouteParameters>();

  const {data: tloMap} = useGetTLOsQuery(
    definedOrSkipToken(exerciseId, deploymentId));

  if (exerciseId && deploymentId) {
    return (
      <PageHolder>
        <DeploymentDetailsGraph
          exerciseId={exerciseId}
          deploymentId={deploymentId}
        />
        <br/>
        <Wrapper>
          <table className='
        bp4-html-table
        bp4-html-table-striped'
          >
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
      </PageHolder>
    );
  }

  return (
    <PageHolder>
      <Wrapper>
        Error: Missing Exercise Id and / or Deployment Id
      </Wrapper>
    </PageHolder>
  );
};

export default DeploymentDetail;
