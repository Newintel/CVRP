import IPoint from './point';

type StrokeColor = CanvasRenderingContext2D['strokeStyle'];

interface IGraph {
    canvas : HTMLCanvasElement,
    ctx : CanvasRenderingContext2D | null,
    addCircle : (
        center : IPoint, radius : number, fill ?: boolean, color ?: StrokeColor
    ) => void,
    addPoint : (p : IPoint, color ?: StrokeColor) => void,
    addPath : (points : IPoint[], color ?: StrokeColor) => void,
    clear : () => void,
}

export default IGraph;
