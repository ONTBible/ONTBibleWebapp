# Les propositions — ce qui est ouvert, par qui, et ce que ça engage

**Décision de l'auteur du 21 septembre 2026.** Toute PR s'inscrit ici, **par
celle qui l'ouvre**, avec ce qu'aucun tableau GitHub ne montre : pourquoi elle
existe, et ==ce qu'elle engage chez les voisins==.

## Pourquoi ce fichier, à côté de `DECISIONS.md`

`DECISIONS.md` porte ce qui est ==tranché== ; celui-ci porte ce qui est
==proposé et attend==. Une PR *est* une proposition — le nom dit ce qu'elle est,
et le couple dit où chaque chose vit.

**Ce qu'un tableau de PR ne donne pas**, et qui manque à chaque relecture :

- **qui l'a ouverte.** Les huit sessions poussent sous le compte `gloiiire` :
  `gh pr list --author @me` rend ==toutes les PR du dépôt==. Trois sessions y
  sont tombées le même jour. ==L'auteur git ne distingue personne== ;
- **pourquoi.** Le titre dit ce que la PR fait, jamais le défaut qu'elle répare
  ni la mesure qui l'a rendue nécessaire ;
- **ce qu'elle engage.** C'est la règle du `CLAUDE.md` racine — *demander ce que
  ce travail change pour les autres dépôts* — et rien ne la portait. ==Un
  changement de forme dans une réponse casse une plateforme qui n'est pas celle
  qu'on regarde en le faisant.==

## Ce que ça coûte : rien

==L'entrée voyage dans la PR qu'elle décrit.== On l'écrit sur la branche qu'on
vient de pousser, avant d'ouvrir la PR — pas de commit de plus, pas de CI de
plus, pas de fusion supplémentaire à attendre.

C'est la différence avec la déclaration d'un worktree, qui coûte un aller-retour
parce qu'elle ne s'attache à aucun travail en cours.

## La forme

    ## #NNN · le titre
    
        ouverte le   la date, par le RÔLE — jamais un nom de session
        vers         la branche de base
        état         ouverte · fusionnée le … · abandonnée le …
    
    **Pourquoi.** Le défaut, et la mesure qui l'a rendu visible.
    
    **Ce que ça engage.** Ce qui traverse vers les autres dépôts, ce qui
    devient irréversible, ce qu'une autre session devra reprendre.
    
    **Pour la relire.** Ce qu'il faut savoir qu'on ne devinerait pas.

==On ne retire pas une entrée quand la PR est fusionnée== : on change son état
et on date. Une proposition abandonnée reste, avec le motif — ==c'est souvent
elle qui a le plus à apprendre==.

**Et on n'écrit pas le « pourquoi » d'une PR qu'on n'a pas ouverte.** Une entrée
peut donc porter ==*à écrire par qui l'a ouverte*== : ce trou-là est une
information, et il se voit.

---

## #155 · Déclarer son worktree, puisque rien ne le prouve

    ouverte le   21 septembre 2026, par la manageuse
    vers         main
    état         ouverte

**Pourquoi.** Rien ne prouve qu'une session tient un worktree — les trois pistes
mesurables échouent. Le site est le cas qui le montre le mieux : il travaille la
branche `inscrire-l-appui-long-au-journal` **depuis son arbre principal**, et le
worktree du même nom est **non réclamé** et en retard d'un commit sur cette même
branche. Un worktree non réclamé peut porter une branche activement tenue.

**Ce que ça engage.** Tronc commun — texte identique dans les trois dépôts. Le
contrôle vit dans le vault ; le site porte la règle.

**Pour la relire.** La table déclare un poste, pas une branche : l'arbre
principal du site a porté **trois branches en une matinée**, ce qui a emporté la
décision.

## #153 · Deux corrections à l'entrée du 18

    ouverte le   20 septembre 2026, par la manageuse
    vers         main
    état         ouverte

**Pourquoi.** Une clause n'avait jamais été écrite alors que j'avais rapporté
qu'elle l'était, et un ajout est venu après coup. Voir l'entrée jumelle, #125
au vault.

**Ce que ça engage.** Tronc commun. Rien de technique.

**Pour la relire.** Le journal de `main` porte **sept titres en double** —
conséquence d'avoir fusionné sept PR qui se recouvraient. Le site les
dédoublonnera en une passe quand sa file sera vide, en ne retirant que les blocs
identiques. Ne pas le faire depuis une autre PR.

## #137 · Porter l'entrée de la nuit du 11

    ouverte le   11 septembre 2026
    vers         main
    état         ouverte, en brouillon

*À écrire par qui l'a ouverte.* Le site note qu'elle se reprend **après #134**,
pour que l'ordre des heures soit juste.
