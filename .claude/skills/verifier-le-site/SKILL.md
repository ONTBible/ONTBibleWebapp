---
name: verifier-le-site
description: Regarder le site tourner avant de dire qu'un travail est fini — monter le serveur, capturer une page, éprouver la composition. À employer dès qu'on touche à une page, un composant ou une feuille de style, et avant d'annoncer à Gloire que c'est fait ; il vérifie sur 127.0.0.1, pas sur ce qu'on lui décrit.
---

# Voir avant de dire que c'est fait

« Ça compile » ne dit presque rien de ce site. Une échelle typographique qui
s'inverse sous 1024 px compile, passe les tests, et se déploie. Un lien de prose
sans soulignement aussi. Ces deux défauts ont vécu des semaines.

Ce qui suit est la chaîne qui les attrape, dans l'ordre où elle se casse quand
on saute une étape. Le *pourquoi* de chaque outil est au §7 bis du `CLAUDE.md` —
ceci en est la manœuvre.

## 1. Le serveur doit vivre à côté de ses voisins

Le site compile contre `../ONTBibleApp/pipeline` et embarque `../ONTBibleApp/dist/`
par **chemin relatif**. Un `git worktree` monté ailleurs échoue donc avant tout
le reste, sur un message de `cargo metadata` qui ne parle ni du site ni du
pipeline :

    failed to load source for dependency `ont-pipeline`

**Un worktree de ce dépôt reste sous `~/ONTBible/`.** Par exemple
`~/ONTBible/ONTBibleWebapp-main` — voisin des deux autres dépôts, donc
compilable.

    git -C ~/ONTBible/ONTBibleWebapp worktree add --detach ~/ONTBible/ONTBibleWebapp-main origin/main
    cd ~/ONTBible/ONTBibleWebapp-main && ./scripts/linker-local.sh
    cargo leptos watch

`linker-local.sh` **une fois par arbre** : le linker d'Apple plante sur le
corpus embarqué, et l'erreur n'oriente vers rien. Chaque worktree a son propre
`.cargo/config.toml`, gitignoré.

## 2. Avant de regarder : mettre à jour, et le dire

Une fusion ne descend pas toute seule. Après un `gh pr merge`, faire le
`git pull --ff-only` **avant** d'annoncer que c'est fusionné — sinon Gloire
relance son serveur, voit la version d'avant, et croit que le travail n'existe
pas. C'est arrivé.

Et si une autre session tient l'arbre principal, ne pas le lui prendre : monter
un worktree (§1) et lancer le serveur depuis là.

## 3. La composition française, après toute page écrite

    ./scripts/verifier-composition.py

Il lit les pages **rendues** et échoue sur toute ponctuation double qui peut
tomber à la ligne. C'est la seule vérification qui voie à la fois la prose du
site (littéraux Rust), les chaînes du corpus et les littéraux coupés en deux par
une continuation de ligne. Les insécables s'écrivent `\u{202f}` devant `; ! ? »`
et `\u{a0}` devant `:`.

## 4. Voir la page, vraiment

**Grand écran** — `./scripts/apercu.py <dossier> nom=/fr/la-page`. Lancer
`./scripts/dev-sync-empreintes.sh` **avant**, sinon on juge le style du dernier
redémarrage complet et l'on débat d'un rendu qui n'est pas celui du code.

**Téléphone** — `ATTENTE=25 ./scripts/sim.sh /fr/la-page`. C'est la seule
vérification qui vaille pour tout ce qui dépend de la largeur : QuickLook rend à
une fenêtre fixe et large, donc les requêtes média y voient toujours un grand
écran. Sous 20 secondes d'attente, la capture est **blanche** — et une page
blanche ressemble beaucoup trop à une page cassée.

**Une page longue ne rentre dans aucun des deux.** Les deux outils ne voient que
le premier écran. Pour une section qui vit au sixième défilement : retirer les
autres `<section>` du HTML servi, poser le fichier dans `target/site/` — servi à
la racine en développement — puis le capturer. C'est ainsi que la césure des
versets et les lettrines décapitées ont été vues ; aucune ne se voyait dans le
code.

## 5. Ce que le rendu ne dit pas, et qu'il faut demander autrement

- **Les marques.** Un intraduisible passe par `Terme`, un nom propre hébreu par
  `Nom`. Jamais `<b>` ni `<i>` : le premier donne du gras sans couleur, le
  second de l'italique, et les deux mentent sur ce que le mot est.
- **Les lettrines ne commencent jamais par « Il ».** `::first-letter` emporte la
  lettre seule : un `I` grandi devient un trait vertical, et le lecteur lit
  « l y a » avec une barre devant. Le défaut est invisible dans le code, où
  « Il » est écrit correctement.
- **Une couleur se mesure, elle ne se regarde pas.** Sur une rampe dérivée au
  calcul, une dérive de teinte se relève à la pipette.

## 6. Compter sur le HTML brut, pas sur une extraction hâtive

Relever un libellé en dépouillant les balises à coups de `sed` ou de regex donne
des faux, dans les deux sens : l'îlot de données que Leptos sérialise porte les
noms internes, et une regex trop gourmande avale le contenu qu'on cherche. Six
fois en deux jours, un instrument mal réglé a raconté une donnée fausse.

Lire le HTML brut, ou dépouiller en retirant d'abord les `<script>`. Et devant
un résultat surprenant : **soupçonner la mesure avant la donnée.**

## 7. Ne pas conclure sur le local seul

Le site en ligne se construit depuis `origin/main`, par GitHub, sans passer par
la machine. Local et production peuvent donc être justes chacun de leur côté et
montrer deux choses différentes. Quand c'est la production qui compte :

    curl -s https://ontbible.com/fr/la-page | ...
