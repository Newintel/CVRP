const Informations = () => {
  const infosDiv = document.createElement('div');
  infosDiv.className = 'm-auto w-auto';

  const label = document.createElement('div');
  label.textContent = 'Distance';

  const distance = document.createElement('div');
  distance.className = 'border border-dark p-1';
  distance.id = 'distance';

  infosDiv.appendChild(label);
  infosDiv.appendChild(distance);

  return infosDiv;
};

export default Informations;
