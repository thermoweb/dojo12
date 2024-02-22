# Douzième dojo (22/02/24) : kata Cupcake

## Énoncé du problème

Enoncé d'origine : <https://codingdojo.org/kata/cupcake/>

Ce kata a été créé à l'origine pour mettre en œuvre le motif décorateur et composite.

Écrivez un programme permettant de construire de nombreux gâteaux avec de nombreuses garnitures comme : « Cupcake au chocolat et aux noix » Ou « 🧁 au noir 🍫 et 🥜 et 🍬 ». Attention, l'ordre de garniture est très important.

Écrivez une fonction ou une méthode qui peut afficher le nom du gâteau.

Écrivez une fonction qui peut afficher le prix d'un gâteau. Le prix est composé du prix de base du gâteau et du prix de la garniture.

### Lot de gâteaux non frais

Il est désormais possible de réaliser un paquet de gâteaux. Le prix d'un lot est 10% inférieur au prix de chaque gâteau.

Il est possible de constituer un paquet de paquets avec des gâteaux simples.

### Cas de test suggérés

En pseudocode pour construire un Cupcake avec du chocolat, des noix et du sucre vous écrirez

```javascript
var myCake = Sugar(Nuts(Chocolate(Cupcake())))
```

En tapant, on peut commencer à tester :

- Je peux mettre un Cupcake dans une variable de type Cake

#### À propos de la fonction ou de la méthode de nom

- La fonction de nom doit renvoyer « 🧁 »
- La fonction de nom doit renvoyer « 🍪 »
- La fonction de nom doit renvoyer « 🧁 avec 🍫 »
- La fonction de nom doit renvoyer « 🍪 avec 🍫 »
- La fonction de nom doit renvoyer « 🍪 avec 🍫 et 🥜 »
- La fonction de nom doit renvoyer « 🍪 avec 🥜 et 🍫 »

#### À propos de la fonction ou de la méthode de prix

- La fonction de prix devrait renvoyer 1 $ pour « 🧁 »
- La fonction de prix devrait renvoyer 2 $ pour « 🍪 »
- La fonction de prix devrait renvoyer 1,1 $ pour « 🧁 avec 🍫 »
- La fonction de prix devrait renvoyer 2,1 $ pour « 🍪 avec 🍫 »
- La fonction de prix devrait renvoyer 2,2 $ pour « 🍪 avec 🥜 »

#### Paquet

- Nous pouvons créer un lot avec 1 cupcake et vérifier le prix ou la description.
- Nous pouvons créer un pack avec 1 cupcake et 1 cookie et vérifier le prix ou la description.
- Nous pouvons créer un lot avec 2 cupcakes et 1 cookie et vérifier le prix ou la description.
- Nous pouvons créer un lot avec 1 lot de 2 gâteaux et 1 cupcake et vérifier le prix ou la description.
- Nous pouvons créer un lot avec de nombreux lots et de nombreux gâteaux et vérifier le prix ou la description.

## Ouvrir le code

Ce dépôt contient la configuration nécessaire pour faire le kata directement dans un devcontainer. Le dossier peut être ouvert directement dans un VSCode avec l'extension `Dev Containers` installée.

Une fois le projet chargé, vous pouvez modifier le programme principal situé dans le fichier `src/main.rs`. Pour lancer le programme, il suffit d'exécuter la commande suivante :

```bash
cargo run
```

## Lancer les tests

Pour exécuter les tests, lancer la commande suivante :

```bash
cargo test
```

Pour lancer _uniquement_ les tests ignorés sans éditer le fichier, vous pouvez lancer la commande suivante :

```bash
cargo test -- --ignored
```

Pour lancer _tous_ les tests, vous pouvez utiliser la commande suivante :

```bash
cargo test -- --include-ignored
```

Pour lancer uniquement un test spécifique, par exemple `useless_test`, utilisez cette commande :

```bash
cargo test useless_test
```

Si le test à lancer est _ignoré_, vous pouvez lancer la commande suivante :

```bash
cargo test always_fail -- --ignored
```

Pour en apprendre plus sur les tests en Rust, référez vous à la [documentation des tests][rust-tests].

[rust-tests]: https://doc.rust-lang.org/book/ch11-00-testing.html
