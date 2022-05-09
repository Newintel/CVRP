import { Attributes } from 'types';


/**
 * Fonction permettant d'ajouter des attributs à un élément HTML
 * @param {Element} element l'élément auquel on ajoute les attributs
 * @param {Attributes} attributes objet contenant les attributs
 */
const addAttributes = (element : Element, attributes : Attributes) => {
  Object.entries(attributes).forEach(([attribute, value]) => {
    element.setAttribute(attribute, value);
  });
};

export { addAttributes };
