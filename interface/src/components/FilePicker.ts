interface IProps {
  onChange : EventListener,
  onValidate : () => void,
}


/**
 * Crée un composant contenant un input pour sélectionner
 * un fichier et un bouton pour valider la sélection
 * @param onChange action à effectuer lorsqu'un nouveau fichier est choisi
 * @param onValidate action à effectuer lorsqu'on clique sur le bouton valider
 */
const FilePicker = ({ onChange, onValidate } : IProps) => {
  const parent = document.createElement('div');
  parent.className = 'd-flex justify-content-around';

  const inputGroup = document.createElement('div');
  inputGroup.className = 'input-group';

  const div = document.createElement('div');
  div.className = 'custom-file';

  const input = document.createElement('input');
  input.type = 'file';
  input.className = 'custom-file-input';
  input.addEventListener(
    'change', onChange,
  );

  const btn = document.createElement('div');
  btn.className = 'btn btn-outline-primary';
  btn.textContent = 'Afficher points';
  btn.addEventListener(
    'click', onValidate,
  );

  div.appendChild(input);
  inputGroup.appendChild(div);

  parent.appendChild(inputGroup);
  parent.appendChild(btn);

  // const cam = CVRP.get_clients();

  return parent;
};

export default FilePicker;
