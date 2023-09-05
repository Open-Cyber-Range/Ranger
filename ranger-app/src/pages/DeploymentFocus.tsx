import type React from 'react';
import {useParams} from 'react-router-dom';
import type {DeploymentDetailRouteParameters} from 'src/models/routes';
import {
  useAdminGetDeploymentQuery,
  useAdminGetDeploymentScenarioQuery,
  useAdminGetDeploymentScoresQuery,
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

const DeploymentFocus = () => {
  const {exerciseId, deploymentId} = useParams<DeploymentDetailRouteParameters>();
  useExerciseStreaming(exerciseId);
  const queryArguments = exerciseId && deploymentId ? {exerciseId, deploymentId} : skipToken;
  const {data: scenario} = useAdminGetDeploymentScenarioQuery(queryArguments);
  const {data: deployment} = useAdminGetDeploymentQuery(queryArguments);
  const {data: scores} = useAdminGetDeploymentScoresQuery(queryArguments);

  if (scenario && exerciseId && deploymentId) {
    return (
      <SideBar renderMainContent={activeTab => (
        <>
          {activeTab === 'Graph' && (
            <>
              <TloTable
                exerciseId={exerciseId}
                deploymentId={deploymentId}
                tloMap={scenario.tlos ?? {}}
              />
              <div className='mt-[2rem]'>
                <DeploymentDetailsGraph
                  entities={scenario.entities ?? {}}
                  tlos={scenario.tlos ?? {}}
                  evaluations={scenario.evaluations ?? {}}
                  metrics={scenario.metrics ?? {}}
                  scenarioStart={scenario?.start ?? ''}
                  scenarioEnd={scenario?.end ?? ''}
                  scores={scores ?? []}
                />
              </div>
            </>
          )}
          {activeTab === 'SDL' && (
            <div className='h-[80vh]'>
              <Editor
                value={deployment?.sdlSchema ?? ''}
                defaultLanguage='yaml'
                options={{readOnly: true}}
              />
            </div>
          )}
          {activeTab === 'Accounts' && (
            <div className='text-center '>
              <AccountList
                exerciseId={exerciseId}
                deploymentId={deploymentId}
              />
            </div>
          )}
          {activeTab === 'Entity Selector' && (
            <>
              <EntityConnector exerciseId={exerciseId} deploymentId={deploymentId}/>
              <div className='mt-[2rem]'>
                <EntityTree exerciseId={exerciseId} deploymentId={deploymentId}/>
              </div>
            </>
          )}
          {activeTab === 'Manual Metrics' && (
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
