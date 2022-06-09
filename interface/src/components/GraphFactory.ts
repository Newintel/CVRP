import {
  Attributes, Graph,
} from 'types';
import { Utils } from 'utils';

interface IGraphFactory {
  attributes ?: Attributes,
}

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

  return ({
    canvas,
    ctx,
  });
};

export default graphFactory;
