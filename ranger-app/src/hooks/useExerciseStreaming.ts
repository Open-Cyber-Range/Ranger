import humanInterval from 'human-interval';
import {useEffect, useRef, useState} from 'react';
import {BASE_URL} from 'src/constants';
import type {WebsocketWrapper} from 'src/models/websocket';
import {WebsocketMessageType} from 'src/models/websocket';
import {apiSlice} from 'src/slices/apiSlice';
import type {AppDispatch, RootState} from 'src/store';
import {useAppDispatch} from 'src/store';
import {getWebsocketBase} from 'src/utils';
import {useSelector} from 'react-redux';

const websocketHandler = (
  dispatch: AppDispatch,
) => (event: MessageEvent<string>) => {
  const data: WebsocketWrapper = JSON.parse(event.data) as WebsocketWrapper;
  switch (data.type) {
    case WebsocketMessageType.ExerciseUpdate: {
      const exerciseUpdate = data.content;
      dispatch(
        apiSlice.util.updateQueryData('adminGetExercise',
          data.exerciseId,
          exercise => {
            Object.assign(exercise, exerciseUpdate);
          }));
      break;
    }

    case WebsocketMessageType.Deployment: {
      const deployment = data.content;
      dispatch(
        apiSlice.util
          .updateQueryData('adminGetDeployments',
            deployment.exerciseId,
            deployments => {
              deployments?.push(deployment);
            }));
      break;
    }

    case WebsocketMessageType.DeploymentElement: {
      const deploymentElement = data.content;
      dispatch(
        apiSlice.util
          .updateQueryData('adminGetDeploymentElements', {
            exerciseId: data.exerciseId,
            deploymentId: deploymentElement.deploymentId,
          }, deploymentElements => {
            deploymentElements?.push(deploymentElement);
          }));
      break;
    }

    case WebsocketMessageType.DeploymentElementUpdate: {
      const deploymentElementUpdate = data.content;
      dispatch(
        apiSlice.util
          .updateQueryData('adminGetDeploymentElements', {
            exerciseId: data.exerciseId,
            deploymentId: deploymentElementUpdate.deploymentId,
          }, deploymentElements => {
            const element = deploymentElements?.find(
              deploymentElement =>
                deploymentElement.id === deploymentElementUpdate.id,
            );
            if (element) {
              Object.assign(element, deploymentElementUpdate);
            }
          }));
      break;
    }

    case WebsocketMessageType.Score: {
      const score = data.content;
      dispatch(
        apiSlice.util
          .updateQueryData('adminGetDeploymentScores', {
            exerciseId: score.exerciseId,
            deploymentId: score.deploymentId,
          },
          scores => {
            scores?.push(score);
          }));
      break;
    }

    default: {
      break;
    }
  }
};

const useExerciseStreaming = (exerciseId?: string) => {
  const dispatch = useAppDispatch();
  const websocket = useRef<WebSocket | undefined>();
  const [trigger, setTrigger] = useState<boolean>(true);
  const token = useSelector((state: RootState) => state.user.token);
  useEffect(() => {
    if (!token || !exerciseId) {
      return;
    }

    if (websocket.current === undefined
        || websocket.current.readyState !== WebSocket.OPEN
    ) {
      websocket.current = new WebSocket(
        `${getWebsocketBase()}${BASE_URL}/admin/exercise/${exerciseId}/websocket`,
        `${token}`,
      );
      const thisInstance = websocket.current;
      thisInstance.addEventListener('message', websocketHandler(dispatch));
      let timeout: number | undefined;
      thisInstance.addEventListener('close', () => {
        timeout = setTimeout(() => {
          if (websocket.current?.readyState !== WebSocket.OPEN) {
            setTrigger(current => !current);
          }
        }, humanInterval('3 seconds'));
      });

      return () => {
        if (timeout) {
          clearTimeout(timeout);
        }

        thisInstance.close();
      };
    }
  }, [dispatch, exerciseId, trigger, token, setTrigger]);
};

export default useExerciseStreaming;
