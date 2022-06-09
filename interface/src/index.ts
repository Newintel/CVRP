import {
  buttonsFactory, filePicker, graphFactory, informationsFactory,
} from 'components';
import { InfosToSet } from 'components/Informations';
import {
  CVRP, wasm_init,
} from 'cvrp';
import { BootstrapColor } from 'types';
import {
  displayRandomPath, displaySA, displayTabuResult, readData,
} from '__cvrp__';

// Display wasm errors in console.error
wasm_init();

const root = document.querySelector('#root');

const MAX_TRUCK_WEIGHT = 100;
const FACTOR = 5;
const OFFSET = 10;
const DEFAULT_ITERATIONS = 100;

const DEFAULT_TABU_SIZE = 30;

const DEFAULT_INITIAL_T = 3;
const DEFAULT_ITERATIONS_PER_T = 20;
const DEFAULT_MU = .1;
const DEFAULT_TEMP_CHANGES = 5;

// Create cvrp global
const cvrp = new CVRP(MAX_TRUCK_WEIGHT, FACTOR, OFFSET);

// Create graph
const graph = graphFactory({
  attributes: {
    class: 'border m-5',
    width: FACTOR * 100 + OFFSET,
    height: FACTOR * 100 + OFFSET,
  },
});

const {
  global, setInfo, getInfo, getNeighborhoodStructs,
} = informationsFactory({
  [InfosToSet.TABU_ITERATIONS]: DEFAULT_ITERATIONS,
  [InfosToSet.TABU_SIZE]: DEFAULT_TABU_SIZE,
  [InfosToSet.SA_MU]: DEFAULT_MU,
  [InfosToSet.SA_T_CHANGES]: DEFAULT_TEMP_CHANGES,
  [InfosToSet.SA_ITERATIONS]: DEFAULT_ITERATIONS_PER_T,
  [InfosToSet.SA_INITIAL_T]: DEFAULT_INITIAL_T,
});

// nav
const buttons = buttonsFactory({
  Random: {
    color: BootstrapColor.SUCCESS,
    onClick: displayRandomPath({
      cvrp,
      graph,
      setInfo,
    }),
  },
  Tabou: {
    color: BootstrapColor.WARNING,
    onClick: displayTabuResult({
      cvrp,
      graph,
      setInfo,
      getInfo,
      getNeighborhoodStructs,
    }),
  },
  Recuit: {
    color: BootstrapColor.DANGER,
    onClick: displaySA({
      cvrp,
      graph,
      setInfo,
      getInfo,
      getNeighborhoodStructs,
    }),
  },
});

const canvasAndButtons = document.createElement('div');
canvasAndButtons.className = 'd-flex justify-content-between mb-2';

// Add graph and filePicker components
canvasAndButtons.appendChild(global);
canvasAndButtons.appendChild(graph.canvas);
canvasAndButtons.appendChild(buttons);

root!.appendChild(canvasAndButtons);
root!.appendChild(filePicker({
  onChange: readData(cvrp),
  onValidate: () => cvrp.display(graph.ctx!, graph.canvas),
}));
