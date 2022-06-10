import { all_labels, NeighborhoodStruct } from 'cvrp';

export enum InfosToSet {
  // Tabu
  TABU_ITERATIONS = 'Itérations',
  TABU_SIZE = 'Taille liste taboue',
  // Simulated annealing
  SA_MU = 'µ',
  SA_ITERATIONS = 'Itérations par température',
  SA_INITIAL_T = 'Température initiale',
  SA_T_CHANGES = 'Changements de température',
}

const labelsToSet = {
  Tabou: [InfosToSet.TABU_ITERATIONS, InfosToSet.TABU_SIZE] as const,
  Recuit: [
    InfosToSet.SA_INITIAL_T,
    InfosToSet.SA_ITERATIONS,
    InfosToSet.SA_MU,
    InfosToSet.SA_T_CHANGES,
  ] as const,
};

type props = {
  [key in InfosToSet] : number
}

const Informations = (props : props) => {
  const globalDiv = document.createElement('div');
  globalDiv.className = 'd-flex flex-column justify-content-around';

  const labels : string[] = all_labels();

  const components : { [ key : string ] : HTMLInputElement } = {};

  const accordion = document.createElement('div');
  accordion.className = 'accordion overflow-visible';

  Object.entries(labelsToSet).forEach(([
    key, value,
  ] :
    [ string, readonly string[]
]) => {
    const item = document.createElement('div');
    item.className = 'accordion-item';
    accordion.append(item);

    const header = document.createElement('div');
    header.className = 'accordion-header';
    item.appendChild(header);

    const button = document.createElement('button');
    button.textContent = key;
    button.type = 'button';
    button.className = 'accordion-button collapsed';
    header.appendChild(button);


    const collapse = document.createElement('div');
    collapse.className = 'accordion-collapse collapse';
    item.appendChild(collapse);

    const divBody = document.createElement('div');
    divBody.className = 'accordion-body';
    collapse.append(divBody);

    button.addEventListener('click', () => {
      if (button.classList.contains('collapsed')) {
        button.classList.remove('collapsed');
      } else {
        button.classList.add('collapsed');
      }
      if (collapse.classList.contains('show')) {
        collapse.classList.remove('show');
      } else {
        collapse.classList.add('show');
      }
    });

    value.forEach(v => {
      const infosDiv = document.createElement('div');
      infosDiv.className = 'input-group';
      divBody.appendChild(infosDiv);

      const labelDiv = document.createElement('span');
      labelDiv.textContent = v;
      labelDiv.className = 'input-group-text';
      infosDiv.appendChild(labelDiv);

      const infoDiv = document.createElement('input');
      infoDiv.className = 'form-control';
      infoDiv.placeholder = '' + props[v as InfosToSet];
      infoDiv.addEventListener('change', ev => {
        const target = ev.target as HTMLInputElement;
        if (isNaN(parseInt(target.value))) {
          alert(`Le champ ${v} devrait être un nombre (int / float)`);
          target.value = '';
        }
      });
      infosDiv.appendChild(infoDiv);

      components[v] = infoDiv;
    });
  });

  globalDiv.appendChild(accordion);

  const subDiv = document.createElement('div');
  globalDiv.appendChild(subDiv);

  labels.forEach(label => {
    const infosDiv = document.createElement('div');
    infosDiv.className = 'input-group mb-1';

    const labelDiv = document.createElement('span');
    labelDiv.className = 'input-group-text';
    labelDiv.textContent = label;

    const infoDiv = document.createElement('input');
    infoDiv.className = 'form-control';
    infoDiv.id = label;

    infoDiv.disabled = true;
    infoDiv.readOnly = true;

    infosDiv.appendChild(labelDiv);
    infosDiv.appendChild(infoDiv);

    subDiv.appendChild(infosDiv);

    components[label] = infoDiv;
  });

  const neighborDiv = document.createElement('div');
  globalDiv.appendChild(neighborDiv);
  const inputs :
    {
      -readonly [ key in keyof typeof NeighborhoodStruct ] ?: HTMLInputElement
    } = {};

  Object.keys(NeighborhoodStruct).forEach(key => {
    if (isNaN(parseInt(key)) === false) {
      return;
    }
    const formDiv = document.createElement('div');
    formDiv.className = 'form-check form-switch';
    neighborDiv.appendChild(formDiv);

    const input = document.createElement('input');
    input.type = 'checkbox';
    input.className = 'form-check-input';
    input.id = key;
    input.checked = true;
    input.setAttribute('role', 'switch');
    formDiv.appendChild(input);
    inputs[key as keyof typeof NeighborhoodStruct] = input;
    input.addEventListener('change', () => {
      if (Object.entries(inputs)
        .filter(([, value]) => value?.checked).length === 0) {
        input.checked = true;
      }
    });

    const label = document.createElement('label');
    label.className = 'form-check-label';
    label.setAttribute('for', key);
    label.textContent = key;
    formDiv.appendChild(label);
  });

  return {
    global: globalDiv,
    setInfo: (prop : string, value : string) => {
      components[prop].value = value;
    },
    getInfo: (prop : InfosToSet) =>
      components[prop].value || ('' + props[prop]),
    getNeighborhoodStructs: () =>
      Object.entries(inputs)
        .filter(([, value]) => value?.checked)
        .map(([key]) => key),
  };
};

export default Informations;
