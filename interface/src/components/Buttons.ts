import { BootstrapColor } from 'types';

interface IProps {
  color : BootstrapColor,
  onClick : EventListener,
}

type ButtonsFactoryProps = {[key in 'Random' | 'Tabou' | 'Recuit'] : IProps}

const ButtonsFactory = (props : ButtonsFactoryProps) => {
  const vert = document.createElement('div');
  vert.className = 'btn-group-vertical';

  Object.entries(props).forEach(([
    name,
    {
      color, onClick,
    },
  ]) => {
    const random = document.createElement('div');
    random.className = `btn btn-${color}`;
    random.textContent = name;
    random.addEventListener('click', onClick);
    vert.appendChild(random);
  });

  return vert;
};


export default ButtonsFactory;
