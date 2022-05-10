import { CVRP } from 'cvrp';
import { Graph, Point } from 'types';

interface IProps {
  cvrp : CVRP,
  graph : Graph
}

const displayCvrp = ({ cvrp, graph } : IProps) : EventListener => () => {
  graph.clear();

  const points : Point[] = cvrp.clients_to_points();

  console.log(points);

  points.forEach(p => graph.addPoint(p));
};

export default displayCvrp;
