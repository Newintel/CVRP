import {
  buttonsFactory, filePicker, graphFactory, informationsFactory,
} from 'components';
import { CVRP, wasm_init } from 'cvrp';
import {
  displayCvrp, displayRandomPath, displayTabuResult, readData,
} from '__cvrp__';

// Display wasm errors in console.error
wasm_init();

const root = document.querySelector('#root');

const MAX_TRUCK_WEIGHT = 100;
const FACTOR = 5;

// Create cvrp global
const cvrp = new CVRP(
  MAX_TRUCK_WEIGHT, FACTOR,
);

// Create graph
const graph = graphFactory({
  attributes: {
    class: 'border m-5', width: FACTOR * 100, height: FACTOR * 100,
  },
});

const labels = ['distance'] as const;
const { global, set: setInfo } = informationsFactory(labels);

// nav
const buttons = buttonsFactory({
  onRandomClick: displayRandomPath({
    cvrp, graph, setInfo,
  }),
  onTabuClick: displayTabuResult({
    cvrp, graph, setInfo,
  }),
});

const canvasAndButtons = document.createElement('div');
canvasAndButtons.className = 'd-flex justify-content-between mb-2';

// Add graph and filePicker components
canvasAndButtons.appendChild(global);
canvasAndButtons.appendChild(graph.canvas);
canvasAndButtons.appendChild(buttons);

root!.appendChild(canvasAndButtons);
root!.appendChild(filePicker({
  onChange: readData(cvrp), onValidate: displayCvrp({ cvrp, graph }),
}));
