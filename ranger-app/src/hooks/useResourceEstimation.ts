/* eslint-disable no-console */
import {useState, useEffect, useCallback} from 'react';
import type {Scenario} from 'src/models/scenario';
import init, {
  parse_and_verify_sdl as parseAndVerifySDL,
} from '@open-cyber-range/wasm-sdl-parser';

type ResourceEstimation = {
  totalRam: number;
  totalCpu: number;
};

const useResourceEstimation = (sdlSchema: string | undefined): ResourceEstimation => {
  const [totalRam, setTotalRam] = useState<number>(0);
  const [totalCpu, setTotalCpu] = useState<number>(0);
  const [isInitialized, setIsInitialized] = useState<boolean>(false);

  const estimateResources = useCallback(async (inputSdlSchema: string | undefined) => {
    if (!inputSdlSchema) {
      return;
    }

    if (!isInitialized) {
      await init();
      setIsInitialized(true);
    }

    let ram = 0;
    let cpu = 0;

    try {
      const parsedSdl = parseAndVerifySDL(inputSdlSchema);
      const scenario = JSON.parse(parsedSdl) as Scenario;

      if (scenario?.infrastructure && scenario?.nodes) {
        for (const nodeName of Object.keys(scenario.infrastructure)) {
          const infraNode = scenario.infrastructure[nodeName];
          const nodeCount = infraNode.count;
          const node = scenario.nodes?.[nodeName];

          if (node?.resources) {
            ram += node.resources.ram * nodeCount;
            cpu += node.resources.cpu * nodeCount;
          }
        }
      }

      ram /= (1024 ** 3);
      setTotalRam(ram);
      setTotalCpu(cpu);
    } catch (error) {
      console.error('SDL Parsing failed:', error);
    }
  }, [isInitialized]);

  useEffect(() => {
    estimateResources(sdlSchema)
      .catch(error => {
        console.error('Error estimating resources:', error);
      });
  }, [sdlSchema, estimateResources]);

  return {totalRam, totalCpu};
};

export default useResourceEstimation;
