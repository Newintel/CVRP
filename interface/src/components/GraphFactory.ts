import {
  Attributes, Graph, Point,
} from 'types';
import { Color } from 'types/draw';
import { Utils } from 'utils';

interface IGraphFactory {
  attributes ?: Attributes,
}

const pointToCoordinates = (p : Point) : [ Point[ 'x' ], Point[ 'y' ] ] => (
  [p.x, p.y]
);

/**
 * Change les coordonnées du point pour mieux correspondre au canvas
 */
const normalizePoint = (p : Point) : Point => ({ x: 5 * p.x, y: 5 * p.y });

/**
 * Crée un canvas
 * @return {Graph} le graphique avec des outils pour sa gestion
 */
const graphFactory = ({ attributes } : IGraphFactory) : Graph => {
  const canvas = document.createElement('canvas');
  if (attributes !== undefined) {
    Utils.addAttributes(
      canvas, attributes,
    );
  }

  const ctx = canvas.getContext('2d');

  const addCircle : Graph[ 'addCircle' ] =
    (
      center : Point, radius : number, fill = false, color = Color.BLACK,
    ) => {
      if (ctx === null) {
        return;
      }

      const _center = normalizePoint(center);

      ctx.beginPath();
      ctx.arc(
        _center.x, _center.y, radius, 0, 2 * Math.PI,
      );

      if (fill) {
        ctx.fillStyle = color;
        ctx.fill();
      }
    };

  const addPoint : Graph[ 'addPoint' ] = (
    point, color = Color.BLACK,
  ) =>
    addCircle(
      point, 3, true, color,
    );


  const addPath : Graph[ 'addPath' ] = (
    points, color = Color.BLACK,
  ) => {
    const len = points.length;
    let i = 0;

    if (ctx === null) {
      return;
    }

    ctx.beginPath();

    while (i < len) {
      ctx.moveTo(...pointToCoordinates(normalizePoint(points[i++])));
      ctx.lineTo(...pointToCoordinates(normalizePoint(points[i % len])));
    }

    ctx.strokeStyle = color;
    ctx.stroke();
  };

  const clear = () => ctx?.clearRect(
    0, 0, canvas.width, canvas.height,
  );

  return ({
    addPoint,
    addCircle,
    canvas,
    ctx,
    addPath,
    clear,
  });
};

export default graphFactory;
