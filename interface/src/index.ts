import {
  buttonsFactory, filePicker, graphFactory,
} from 'components';
import { CVRP, wasm_init } from 'cvrp';
import {
  displayCvrp, displayRandomPath, displayTabuResult, readData,
} from '__cvrp__';

// Display wasm errors in console.error
wasm_init();

const root = document.querySelector('#root');

const C = 100;

// Create cvrp global
const cvrp = CVRP.new(
  C, 5, undefined, 10000,
);

// Create graph
const graph = graphFactory({
  attributes: {
    class: 'border m-5', width: 500, height: 500,
  },
});

// nav
const buttons = buttonsFactory({
  onRandomClick: displayRandomPath({ cvrp, graph }),
  onTabuClick: displayTabuResult({ cvrp, graph }),
});


const canvasAndButtons = document.createElement('div');
canvasAndButtons.className = 'd-flex justify-content-around';

// Add graph and filePicker components
canvasAndButtons.appendChild(graph.canvas);
canvasAndButtons.appendChild(buttons);

root!.appendChild(canvasAndButtons);
root!.appendChild(filePicker({
  onChange: readData(cvrp), onValidate: displayCvrp({ cvrp, graph }),
}));
