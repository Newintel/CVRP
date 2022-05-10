import { filePicker, graphFactory } from 'components';
import { CVRP } from 'cvrp';
import { displayCvrp, readData } from '__cvrp__';


const root = document.querySelector('#root');

const cvrp = CVRP.new();

const graph = graphFactory(
  { attributes: { class: 'border m-5', width: 1000, height: 500 } },
);

root!.appendChild(graph.canvas);

root!.appendChild(filePicker({
  onChange: readData(cvrp),
  onValidate: displayCvrp({ cvrp, graph }),
}));
