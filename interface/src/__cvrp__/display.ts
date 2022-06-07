import { Client, CVRP } from 'cvrp';
import {
  Color, colors, Graph,
} from 'types';

interface IProps {
  cvrp : CVRP,
  graph : Graph,
}

const displayCvrp = ({ cvrp, graph } : IProps) => () => {
  graph.clear();

  const points : Client[] = cvrp.get_clients();

  points.forEach(p => graph.addPoint(
    p, Color[p.i === 0 ? 'RED' : 'BLACK'],
  ));
};

const displayRandomPath = ({ cvrp, graph } : IProps) => () => {
  if (graph.ctx === null) {
    return;
  }
  // displayCvrp({ cvrp, graph })();
  cvrp.random_solution(
    graph.ctx, graph.canvas, colors,
  );
};

const displayTabuResult = ({ cvrp, graph } : IProps) => () => {
  if (graph.ctx === null) {
    return;
  }
  displayCvrp({ cvrp, graph });
  cvrp.tabu_search(
    30, graph.ctx, graph.canvas, colors, 2000,
  );
};

export {
  displayCvrp, displayRandomPath, displayTabuResult,
};
