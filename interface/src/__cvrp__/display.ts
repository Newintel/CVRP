import { Client, CVRP } from 'cvrp';
import {
  Color, colors, Graph,
} from 'types';

interface IProps {
  cvrp : CVRP,
  graph : Graph,
  setInfo : (props : string, value : string) => void,
}

const displayCvrp = ({
  cvrp, graph,
} : Omit<IProps, 'setInfo'>) => () => {
  graph.clear();

  const points : Client[] = cvrp.get_clients();

  points.forEach(p => graph.addPoint(
    p, Color[p.i === 0 ? 'RED' : 'BLACK'],
  ));
};

const displayRandomPath = ({
  cvrp, graph, setInfo,
} : IProps) => () => {
  if (graph.ctx === null) {
    return;
  }
  cvrp.random_solution(
    graph.ctx, graph.canvas, colors, setInfo,
  );
};

const displayTabuResult = ({
  cvrp, graph, setInfo,
} : IProps) => () => {
  if (graph.ctx === null) {
    return;
  }
  displayCvrp({
    cvrp, graph,
  });
  cvrp.tabu_search(
    30, graph.ctx, graph.canvas, colors, 200, setInfo,
  );
};

export {
  displayCvrp, displayRandomPath, displayTabuResult,
};
