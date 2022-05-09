import { Attributes, Graph, Point } from 'types';
import { Color } from 'types/draw';
import { Utils } from 'utils';

interface IGraphFactory {
  attributes ?: Attributes
}

const pointToCoordinates = (p : Point) : [Point['x'], Point['y']] => [p.x, p.y];

/**
 * Crée un canvas
 * @return {Graph} le graphique avec des outils pour sa gestion
 */
const graphFactory = ({ attributes } : IGraphFactory) : Graph => {
  const canvas = document.createElement('canvas');
  if (attributes !== undefined) {
    Utils.addAttributes(canvas, attributes);
  }

  const ctx = canvas.getContext('2d');

  const addCircle = (center : Point, radius : number, fill ?: boolean) => {
    if (ctx === null) {
      return;
    }

    ctx.beginPath();
    ctx.arc(center.x, center.y, radius, 0, 2*Math.PI);

    if (fill) {
      ctx.fill();
    }
  };

  const addPoint : Graph['addPoint'] = point => {
    addCircle(point, 3, true);
  };

  const addPath : Graph['addPath'] = (points, color = Color.BLACK) => {
    const len = points.length;
    let i = 0;

    if (ctx === null) {
      return;
    }

    while ( i < len) {
      ctx.moveTo(...pointToCoordinates(points[i++]));
      ctx.lineTo(...pointToCoordinates(points[i % len]));
    }

    ctx.strokeStyle = color;
    ctx.stroke();
  };

  return ({
    addPoint,
    addCircle,
    canvas,
    ctx,
    addPath,
  });
};

export default graphFactory;
