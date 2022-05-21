interface IProps {
  onRandomClick : EventListener,
  onTabuClick : EventListener,
}

const ButtonsFactory = ({ onRandomClick, onTabuClick } : IProps) => {
  const random = document.createElement('div');
  random.className = 'btn btn-success';
  random.textContent = 'Random';
  random.addEventListener(
    'click', onRandomClick,
  );

  const tabu = document.createElement('div');
  tabu.className = 'btn btn-warning';
  tabu.textContent = 'Tabu';
  tabu.addEventListener(
    'click', onTabuClick,
  );

  const vert = document.createElement('div');
  vert.className = 'btn-group-vertical';

  vert.appendChild(random);
  vert.appendChild(tabu);

  return vert;
};


export default ButtonsFactory;
