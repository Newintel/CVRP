import { CVRP } from 'cvrp';
import {
  Graph,
} from 'types';

interface IProps {
  cvrp : CVRP,
  graph : Graph,
}

interface WithDisplay {
  setInfo : (prop : string, value : string) => void,
}

interface Tabu {
  getIterations : () => number,
  getNeighborhoodStructs : () => string[],
}

interface SA {
  getIterationsByTemp : () => number,
  getMu : () => number,
  getTempChanges : () => number,
  getInitialTemp : () => number,
  getNeighborhoodStructs : () => string[],
}

const displayRandomPath = ({
  cvrp, graph, setInfo,
} : IProps & WithDisplay) => () => {
  if (graph.ctx === null) {
    return;
  }
  cvrp.random_solution(
    graph.ctx, graph.canvas, setInfo,
  );
};

const displayTabuResult = ({
  cvrp, graph, setInfo, getIterations,
  getNeighborhoodStructs,
} : IProps & WithDisplay & Tabu) => () => {
  if (graph.ctx === null) {
    return;
  }
  cvrp.tabu_search(
    30, graph.ctx, graph.canvas, getIterations(), setInfo,
    getNeighborhoodStructs(),
  );
};

const displaySA = ({
  cvrp, graph, setInfo, getIterationsByTemp,
  getMu, getTempChanges, getInitialTemp, getNeighborhoodStructs,
} : IProps & WithDisplay & SA) => () => {
  if (graph.ctx === null) {
    return;
  }
  cvrp.simulated_annealing(
    getInitialTemp(), getTempChanges(), getMu(),
    getIterationsByTemp(), graph.ctx, graph.canvas, setInfo,
    getNeighborhoodStructs(),
  );
};

export {
  displayRandomPath, displayTabuResult, displaySA,
};
