import {useEffect, useRef, useState} from 'react';
import {BASE_URL} from 'src/constants';
import type {WebsocketWrapper} from 'src/models/websocket';
import {WebsocketMessageType} from 'src/models/websocket';
import {apiSlice} from 'src/slices/apiSlice';
import type {AppDispatch} from 'src/store';
import {useAppDispatch} from 'src/store';

const websocketHandler = (
  dispatch: AppDispatch,
) => (event: MessageEvent<string>) => {
  const data: WebsocketWrapper = JSON.parse(event.data) as WebsocketWrapper;

  switch (data.type) {
    case WebsocketMessageType.ExerciseUpdate: {
      const exerciseUpdate = data.content;
      dispatch(
        apiSlice.util.updateQueryData('getExercise',
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
          .updateQueryData('getDeployments',
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
          .updateQueryData('getDeploymentElements', {
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
          .updateQueryData('getDeploymentElements', {
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

    default:
      break;
  }
};

const useExerciseStreaming = (exerciseId?: string) => {
  const dispatch = useAppDispatch();
  const websocket = useRef<WebSocket | undefined>();
  const [connect, setConnect] = useState<boolean>(true);

  useEffect(() => {
    if (
      exerciseId
      && connect
      && (websocket.current === undefined || websocket.current?.CLOSING
      || websocket.current?.CLOSED)
    ) {
      websocket.current = new WebSocket(
        `ws://localhost:3000${BASE_URL}/exercise/${exerciseId}/websocket`,
      );

      websocket.current.addEventListener('message', websocketHandler(dispatch));
      websocket.current.addEventListener('close', () => {
        setConnect(true);
      });

      const thisInstance = websocket.current;
      setConnect(false);
      return () => {
        thisInstance.close();
      };
    }
  }, [dispatch, exerciseId, connect, setConnect]);
};

export default useExerciseStreaming;
