import { graphFactory } from 'components';
import { Point } from 'types';
import { Color } from 'types/draw';


const root = document.querySelector('#root');
const {
  canvas,
  addPoint,
  addPath,
} = graphFactory(
    { attributes: { class: 'border m-5', width: 1000, height: 500 } },
);

root!.appendChild(canvas);
document.body.append(root!);

const points : Point[] = [
  { x: 20, y: 20 },
  { x: 30, y: 20 },
  { x: 50, y: 50 },
];

points.forEach(p => addPoint(p));
addPath(points, Color.BLUE);
