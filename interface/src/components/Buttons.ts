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
    const method = document.createElement('div');
    method.className = `btn btn-${color}`;
    method.textContent = name;
    method.addEventListener('click', onClick);
    vert.appendChild(method);
  });

  return vert;
};


export default ButtonsFactory;
