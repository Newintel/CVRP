import { CVRP } from 'cvrp';
import { Color, Graph, IClient } from 'types';

interface IProps {
  cvrp : CVRP,
  graph : Graph,
}

const displayCvrp = ({ cvrp, graph } : IProps) : EventListener => () => {
  graph.clear();

  const points : IClient[] = cvrp.get_clients();

  points.forEach(p => graph.addPoint(p, Color[p.i === 0 ? "RED" : "BLACK"]));
};

export default displayCvrp;
