import {
  all_labels, CVRP, NeighborhoodStruct,
} from 'cvrp';

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

enum Infos {
  TABU = 'Tabou',
  SA = 'Recuit Simulé',
  STATS = 'Statistiques',
  NEIGHBORHOOD = 'Structures de voisinage',
}

type LTS = {
  [key in Infos] : 'show' | '' | readonly InfosToSet[]
}

const labelsToSet : LTS = {
  [Infos.TABU]: [InfosToSet.TABU_ITERATIONS, InfosToSet.TABU_SIZE] as const,
  [Infos.SA]: [
    InfosToSet.SA_INITIAL_T,
    InfosToSet.SA_ITERATIONS,
    InfosToSet.SA_MU,
    InfosToSet.SA_T_CHANGES,
  ] as const,
  [Infos.STATS]: 'show',
  [Infos.NEIGHBORHOOD]: '',
};

type props = {
  [key in InfosToSet] : number
}

const Informations = (props : props, cvrp : CVRP, default_c : number) => {
  const globalDiv = document.createElement('div');
  globalDiv.className = 'd-flex flex-column justify-content-between';

  const subDiv = document.createElement('div');
  globalDiv.appendChild(subDiv);

  const infosDiv = document.createElement('div');
  infosDiv.className = 'input-group mb-2';
  subDiv.appendChild(infosDiv);

  const labelDiv = document.createElement('span');
  labelDiv.textContent = 'Capacité des camions';
  labelDiv.className = 'input-group-text';
  infosDiv.appendChild(labelDiv);

  const infoDiv = document.createElement('input');
  infoDiv.type = 'number';
  infoDiv.className = 'form-control';
  infoDiv.min = '1';
  infoDiv.valueAsNumber = default_c;
  infoDiv.addEventListener('change', ev => {
    const target = ev.target as HTMLInputElement;
    if (target.value === '' || target.valueAsNumber === 0) {
      alert(`La capacité doit être supérieure à 0`);
      target.valueAsNumber = default_c;
    } else if (cvrp.set_capacity(target.valueAsNumber) === false) {
      target.valueAsNumber = default_c;
    }
  });
  infosDiv.appendChild(infoDiv);

  const labels : string[] = all_labels();

  const components : { [ key : string ] : HTMLInputElement } = {};

  const inputs :
    {
      -readonly [ key in keyof typeof NeighborhoodStruct ] ?: HTMLInputElement
    } = {};

  const accordion = document.createElement('div');
  accordion.className = 'accordion';
  subDiv.appendChild(accordion);

  Object.entries(labelsToSet).forEach(([key, value] ) => {
    const item = document.createElement('div');
    item.className = 'accordion-item';
    accordion.append(item);

    const header = document.createElement('div');
    header.className = 'accordion-header';
    item.appendChild(header);

    const button = document.createElement('button');
    button.textContent = key;
    button.type = 'button';
    button.className = 'accordion-button';
    header.appendChild(button);

    const collapse = document.createElement('div');
    collapse.className = 'accordion-collapse collapse';
    item.appendChild(collapse);

    if (value === 'show') {
      collapse.classList.add('show');
    } else {
      button.classList.add('collapsed');
    }

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

    if (Array.isArray(value)) {
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
          if (target.value !== '' && isNaN(parseInt(target.value))) {
            alert(`Le champ ${v} devrait être un nombre (int / float)`);
            target.value = '';
          }
        });
        infosDiv.appendChild(infoDiv);

        components[v] = infoDiv;
      });
    } else {
      const k : Infos = key as Infos;
      if (k === Infos.STATS) {
        const subDiv = document.createElement('div');
        divBody.appendChild(subDiv);

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
      } else if (k === Infos.NEIGHBORHOOD) {
        const neighborDiv = document.createElement('div');
        divBody.appendChild(neighborDiv);

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
      }
    }
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
