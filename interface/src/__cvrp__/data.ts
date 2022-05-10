import { CVRP } from 'cvrp';

const readData = (cvrp : CVRP) : EventListener => event => {
  const target = event.target as HTMLInputElement;
  const fr = new FileReader();

  if (target.files === null) {
    return;
  }

  fr.onloadend = ev => {
    if (ev.target?.result !== undefined) {
      cvrp.read_data(ev.target.result as string);
    }
  };

  fr.readAsText(target.files[0]);
};

export default readData;
