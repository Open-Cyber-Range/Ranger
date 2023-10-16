import type React from 'react';
import {useParams} from 'react-router-dom';
import type {DeploymentDetailRouteParameters} from 'src/models/routes';
import {
  useAdminGetDeploymentElementsQuery,
  useAdminGetDeploymentQuery,
  useAdminGetDeploymentScenarioQuery,
  useAdminGetDeploymentScoresQuery,
  useAdminGetDeploymentUsersQuery,
} from 'src/slices/apiSlice';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import SideBar from 'src/components/Exercise/SideBar';
import useExerciseStreaming from 'src/hooks/useExerciseStreaming';
import DeploymentDetailsGraph from 'src/components/Scoring/Graph';
import TloTable from 'src/components/Scoring/TloTable';
import {Editor} from '@monaco-editor/react';
import AccountList from 'src/components/Deployment/AccountList';
import EntityConnector from 'src/components/Deployment/EntityConnector';
import MetricScorer from 'src/components/Scoring/MetricScorer';
import EntityTree from 'src/components/Deployment/EntityTree';
import {ActiveTab} from 'src/models/exercise';
import {tryIntoScoringMetadata} from 'src/utils';

const DeploymentFocus = () => {
  const {exerciseId, deploymentId} = useParams<DeploymentDetailRouteParameters>();
  useExerciseStreaming(exerciseId);
  const queryArguments = exerciseId && deploymentId ? {exerciseId, deploymentId} : skipToken;
  const {data: scenario} = useAdminGetDeploymentScenarioQuery(queryArguments);
  const {data: deployment} = useAdminGetDeploymentQuery(queryArguments);
  const {data: scores} = useAdminGetDeploymentScoresQuery(queryArguments);
  const {data: users} = useAdminGetDeploymentUsersQuery(queryArguments);
  const {data: deploymentElements} = useAdminGetDeploymentElementsQuery(queryArguments);

  if (scenario && exerciseId && deploymentId) {
    return (
      <SideBar renderMainContent={activeTab => (
        <>
          {activeTab === ActiveTab.Scores && (
            <>
              <TloTable
                scoringData={tryIntoScoringMetadata(scenario)}
                scores={scores}
                tloMap={scenario?.tlos}
              />
              <div className='mt-[2rem]'>
                <DeploymentDetailsGraph
                  colorsByRole
                  scoringData={tryIntoScoringMetadata(scenario)}
                  scores={scores}
                />
              </div>
            </>
          )}
          {activeTab === ActiveTab.SDL && (
            <div className='h-[80vh]'>
              <Editor
                value={deployment?.sdlSchema ?? ''}
                defaultLanguage='yaml'
                options={{readOnly: true}}
              />
            </div>
          )}
          {activeTab === ActiveTab.Accounts && (
            <div>
              <AccountList
                users={users}
                deploymentElements={deploymentElements}
              />
            </div>
          )}
          {activeTab === ActiveTab.EntitySelector && (
            <>
              <EntityConnector exerciseId={exerciseId} deploymentId={deploymentId}/>
              <div className='mt-[2rem]'>
                <EntityTree exerciseId={exerciseId} deploymentId={deploymentId}/>
              </div>
            </>
          )}
          {activeTab === ActiveTab.UserSubmissions && (
            <MetricScorer
              exerciseId={exerciseId}
              deploymentId={deploymentId}/>
          )}
        </>
      )}/>
    );
  }

  return null;
};

export default DeploymentFocus;
