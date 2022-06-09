import { InfosToSet } from 'components/Informations';
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

interface Metaheuristique {
  getInfo : (prop : InfosToSet) => string,
  getNeighborhoodStructs : () => string[],
}

const displayRandomPath = ({
  cvrp, graph, setInfo,
} : IProps & WithDisplay) => () => {
  if (graph.ctx === null) {
    return;
  }
  cvrp.random_solution(graph.ctx, graph.canvas, setInfo);
};

const displayTabuResult = ({
  cvrp, graph, setInfo, getInfo,
  getNeighborhoodStructs,
} : IProps & WithDisplay & Metaheuristique) => () => {
  if (graph.ctx === null) {
    return;
  }
  cvrp.tabu_search(
    parseInt(getInfo(InfosToSet.TABU_SIZE)),
    graph.ctx,
    graph.canvas,
    parseInt(getInfo(InfosToSet.TABU_ITERATIONS)),
    setInfo,
    getNeighborhoodStructs(),
  );
};

const displaySA = ({
  cvrp, graph, setInfo, getInfo, getNeighborhoodStructs,
} : IProps & WithDisplay & Metaheuristique) => () => {
  if (graph.ctx === null) {
    return;
  }
  cvrp.simulated_annealing(
    parseFloat(getInfo(InfosToSet.SA_INITIAL_T)),
    parseInt(getInfo(InfosToSet.SA_T_CHANGES)),
    parseFloat(getInfo(InfosToSet.SA_MU)),
    parseInt(getInfo(InfosToSet.SA_ITERATIONS)),
    graph.ctx,
    graph.canvas,
    setInfo,
    getNeighborhoodStructs(),
  );
};

export {
  displayRandomPath, displayTabuResult, displaySA,
};
