interface IProps {
  onRandomClick : EventListener,
}

const ButtonsFactory = ({ onRandomClick } : IProps) => {
  const random = document.createElement('div');
  random.className = 'btn btn-success';
  random.textContent = 'Random';
  random.addEventListener(
    'click', onRandomClick,
  );

  const vert = document.createElement('div');
  vert.className = 'btn-group-vertical';

  vert.appendChild(random);

  return vert;
};


export default ButtonsFactory;
