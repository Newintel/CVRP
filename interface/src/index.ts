import { filePicker, graphFactory } from 'components';
import { CVRP, wasm_init } from 'cvrp';
import { displayCvrp, readData } from '__cvrp__';

// Display wasm errors in console.error
wasm_init();

const root = document.querySelector('#root');

// Create cvrp global
const cvrp = CVRP.new();

// Create graph
const graph = graphFactory({ attributes: {
  class: 'border m-5', width: 500, height: 500,
} });

// Add graph and filePicker components
root!.appendChild(graph.canvas);
root!.appendChild(filePicker({ onChange: readData(cvrp), onValidate: displayCvrp({ cvrp, graph }) }));
