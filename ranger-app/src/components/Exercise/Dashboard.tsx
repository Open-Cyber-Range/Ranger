import type React from 'react';
import {useAddDeploymentMutation} from 'src/slices/apiSlice';
import {useTranslation} from 'react-i18next';
import ExerciseForm from 'src/components/Exercise/Form';
import DeploymentList from 'src/components/Deployment/List';
import type {
  Deployment,
  DeploymentForm,
  NewDeployment,
} from 'src/models/deployment';
import {toastSuccess, toastWarning} from 'src/components/Toaster';
import Header from 'src/components/Header';
import useExerciseStreaming from 'src/hooks/useExerciseStreaming';
import AddDialog from 'src/components/Deployment/AddDialog';

import {type Exercise} from 'src/models/exercise';

const DashboardPanel = ({exercise, deployments}:
{exercise: Exercise | undefined;
  deployments: Deployment[] | undefined;
}) => {
  const {t} = useTranslation();
  useExerciseStreaming(exercise?.id);
  const [addDeployment, _newDeployment] = useAddDeploymentMutation();

  const createNewDeployment = (
    name: string,
    deploymentGroup?: string,
  ): [NewDeployment, string] | undefined => {
    if (exercise?.sdlSchema && exercise?.id) {
      return [{
        name,
        sdlSchema: exercise.sdlSchema,
        deploymentGroup,
      }, exercise.id];
    }

    toastWarning(t('deployments.sdlMissing'));
  };

  const createPromises = (
    count: number,
    exerciseId: string,
    deployment: NewDeployment,
  ) => {
    const promises = [];
    if (count < 2) {
      promises.push(
        addDeployment({newDeployment: deployment, exerciseId}),
      );
    } else {
      for (let index = 0; index < count; index += 1) {
        promises.push(
          addDeployment({newDeployment: {
            ...deployment,
            name: `${deployment.name}-${index}`,
          }, exerciseId}),
        );
      }
    }

    return promises.map(async promise =>
      promise.unwrap()
        .then(newDeployment => {
          toastSuccess(
            t(
              'deployments.addingSuccess',
              {newDeploymentName: newDeployment.name},
            ),
          );
        })
        .catch(() => {
          toastWarning(t('deployments.addingFail'));
        }));
  };

  const addNewDeployment = async (
    {count, deploymentGroup, name}: DeploymentForm,
  ) => {
    const deploymentInfo = createNewDeployment(name, deploymentGroup);
    if (deploymentInfo) {
      const [deployment, exerciseId] = deploymentInfo;

      const promises = createPromises(
        count,
        exerciseId,
        deployment,
      );
      await Promise.all(promises);
    }
  };

  if (exercise && deployments) {
    return (
      <>
        <ExerciseForm exercise={exercise}/>
        <br/>
        <Header
          headerTitle={t('deployments.title')}
          buttonTitle={t('deployments.add')}
          onSubmit={async (value: DeploymentForm) => {
            await addNewDeployment(value);
          }}
        >
          <AddDialog
            title={t('deployments.title')}
          />
        </Header>
        <DeploymentList deployments={deployments ?? []}/>
      </>
    );
  }

  return null;
};

export default DashboardPanel;
