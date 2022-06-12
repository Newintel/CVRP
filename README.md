<div align="center">
  <h1><code>CVRP</code></h1>

  <strong>Par Thomas PISCIONE et Franck LABRACHERIE</strong>
</div>

Le code rust se situe dans le dossier cvrp/, et le code web dans le dossier interface/

## Execution simple
Ouvrir le fichier interface/dist/index.html

## Prérequis debug
1. <a href="https://www.rust-lang.org/tools/install">Installer  Rust</a> (OS Linux conseillé pour une plus grande facilité en terme de gestion de packages pour les installations)
2. <a href="https://rustwasm.github.io/wasm-pack/installer/">Installer  wasm-pack</a>
3. Sur OS Linux:
```
$ sudo apt install build-essential
``` 
4. Installer npm
5. Dans le répertoire interface/ :
```
$ npm install
``` 

## Lancement debug

Pour lancer taper la commande 
```
bash launch.bash
```

Ensuite, aller à l'adresse donnée dans le terminal (normalement <http://localhost:8080>)