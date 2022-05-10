import IPoint from './point';

type StrokeColor = CanvasRenderingContext2D['strokeStyle'];

interface IGraph {
    canvas : HTMLCanvasElement,
    ctx : CanvasRenderingContext2D | null,
    addCircle : (center : IPoint, radius : number, fill ?: boolean) => void,
    addPoint : (p : IPoint) => void,
    addPath : (points : IPoint[], color ?: StrokeColor) => void,
    clear : () => void
}

export default IGraph;
