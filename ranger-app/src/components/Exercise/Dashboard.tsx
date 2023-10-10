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
import DeploymentList from 'src/components/Deployment/List';

const DashboardPanel = ({exercise, deployments}:
{exercise: Exercise | undefined;
  deployments: Deployment[] | undefined;
}) => {
  const {t} = useTranslation();
  const [addDeployment, _newDeployment] = useAdminAddDeploymentMutation();
  const [isModified, setIsModified] = useState(false);
  const [isAddDialogOpen, setIsAddDialogOpen] = useState(false);

  const createNewDeployment = (
    deploymentForm: DeploymentForm,
  ): [NewDeployment, string] | undefined => {
    if (exercise?.sdlSchema && exercise?.id && deploymentForm.start && deploymentForm.end) {
      let updatedSchema = exercise.sdlSchema.replace(
        /start: \d{4}-\d{2}-\d{2}t\d{2}:\d{2}:\d{2}z/i,
        `start: ${deploymentForm.start}`,
      );

      updatedSchema = updatedSchema.replace(
        /end: \d{4}-\d{2}-\d{2}t\d{2}:\d{2}:\d{2}z/i,
        `end: ${deploymentForm.end}`,
      );

      return [{
        name: deploymentForm.name,
        sdlSchema: updatedSchema,
        deploymentGroup: deploymentForm.deploymentGroup,
        groupName: deploymentForm.groupName,
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
    for (let index = 0; index < count; index += 1) {
      promises.push(
        addDeployment({newDeployment: {
          ...deployment,
          name: count < 2 ? deployment.name : `${deployment.name}-${index}`,
        }, exerciseId}),
      );
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
    deploymentForm: DeploymentForm,
  ) => {
    const deploymentInfo = createNewDeployment(deploymentForm);
    if (deploymentInfo) {
      const [deployment, exerciseId] = deploymentInfo;

      const promises = createPromises(
        deploymentForm.count,
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
        <div className='justify-end items-center pb-4 mt-[2rem]'>
          {deployments === null ? (
            <span className='text-lg text-gray-400'>{t('deployment.empty')}</span>
          ) : (
            <DeploymentList deployments={deployments}/>
          )}
        </div>
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
