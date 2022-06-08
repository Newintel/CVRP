const Informations = (props : readonly string[]) => {
  const globalDiv = document.createElement('div');
  globalDiv.id = 'info';

  const components : {[key in typeof props[number]] : HTMLInputElement} = {};

  props.forEach(label => {
    const infosDiv = document.createElement('div');
    infosDiv.className = 'input-group';

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

    globalDiv.appendChild(infosDiv);

    components[label] = infoDiv;
  });


  return { global: globalDiv, set: (
    prop : string, value : string,
  ) => components[prop].textContent = value };
};

export default Informations;
