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
  displayCvrp({ cvrp, graph })();
  cvrp.random_solution();
  let i = 0;
  (cvrp.get_routes() as number[][])
    .forEach(path => {
      graph.addPath(
        path.map(index => cvrp.get_client(index) as Client),
        colors[i % colors.length],
      );
      console.log(colors[i % colors.length]);
      i += 1;
    });
};

export { displayCvrp, displayRandomPath };
