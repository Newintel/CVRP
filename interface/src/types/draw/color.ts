import { Color } from 'cvrp';

type color = {
    [ K in keyof typeof Color ] -?: K extends number ? never : string
}

const eColor : color = {
  BLACK: '#000',
  RED: '#F00',
  GREEN: '#0F0',
  BLUE: '#00F',
  YELLOW: '#FF0',
  PURPLE: '#F0F',
  CYAN: '#0FF',
};

export const colors = Object.values(eColor) as String[];

export default eColor;
