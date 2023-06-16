import type React from 'react';
import {useAdminAddDeploymentMutation} from 'src/slices/apiSlice';
import {useTranslation} from 'react-i18next';
import ExerciseForm from 'src/components/Exercise/Form';
import type {
  Deployment,
  DeploymentForm,
  NewDeployment,
} from 'src/models/deployment';
import {toastSuccess, toastWarning} from 'src/components/Toaster';
import AddDialog from 'src/components/Deployment/AddDialog';
import {type Exercise} from 'src/models/exercise';
import {useState} from 'react';
import {Alert, Button} from '@blueprintjs/core';

const DashboardPanel = ({exercise, deployments}:
{exercise: Exercise | undefined;
  deployments: Deployment[] | undefined;
}) => {
  const {t} = useTranslation();
  const [addDeployment, _newDeployment] = useAdminAddDeploymentMutation();
  const [isModified, setIsModified] = useState(false);
  const [isAddDialogOpen, setIsAddDialogOpen] = useState(false);

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
        <ExerciseForm
          exercise={exercise}
          onContentChange={isChanged => {
            setIsModified(isChanged);
          }}
        >
          <Button
            large
            intent='success'
            onClick={() => {
              setIsAddDialogOpen(true);
            }}
          >
            {t('deployments.create')}
          </Button>
        </ExerciseForm>
        <Alert
          isOpen={isModified && isAddDialogOpen}
          onConfirm={() => {
            setIsModified(false);
          }}
        >
          <p>{t('exercises.sdlNotSaved')}</p>
        </Alert>
        <AddDialog
          isOpen={!isModified && isAddDialogOpen}
          title={t('deployments.title')}
          onCancel={() => {
            setIsAddDialogOpen(false);
          }}
          onSubmit={async deployment => {
            await addNewDeployment(deployment);
            setIsAddDialogOpen(false);
          }}
        />
      </>
    );
  }

  return null;
};

export default DashboardPanel;
