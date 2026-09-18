## 10 septembre 2026 — le vault a bougé onze fois, et une seule chose traverse vraiment

**Onze commits sur `ecrire-la-premiere-khuqqah`** — `125df42..ca8cbeb`, soit de
`7c36003` à `ca8cbeb` inclus. *(La notation compte : `7c36003..ca8cbeb` en rend
**dix**, la borne gauche étant exclue. Ce journal a déjà une entrée sur les
opérateurs de plage qui ne posent pas la même question ; elle vaut aussi quand
on cite une plage dans une phrase.)* Un
**Shem** séparé en deux, environ cent vingt noms propres rendus touchables, une
casse alignée, un `CLAUDE.md` qui gagne une section, et vingt-trois versets de
corps repris.

La synchronisation a été faite après coup, et c'est déjà la première chose à
retenir : **aucun des onze commits n'a demandé ce que la journée changeait pour
les voisins.** La règle en tête de ce fichier n'est pas une formalité de fin de
travail — elle est ce qui a trouvé le défaut ci-dessous, et rien d'autre ne
l'aurait trouvé.

### Le site colore deux cent quatorze Shemot sur deux cent quinze, et n'en ouvre aucune

**Le défaut, mesuré et non déduit.** Le pipeline émet deux index séparés :

    dist/glossary.json   139 entrées   les intraduisibles
    dist/shemot.json     215 entrées   les noms propres

Leur intersection est de **une** entrée, et cette entrée-là est un défaut à
elle seule — voir juste après. Or `ONTBibleWebapp` n'embarque que
`corpus.json`, `glossary.json`, `occurrences.json` et `search.json` — il n'y a
**aucune occurrence de « shemot » dans tout son `src/`**. La page de fiche
résout contre le seul glossaire (`src/api.rs:276`), et ce qu'elle ne trouve pas
tombe sur la branche `_ => view! { <Absente /> }` (`src/interface/pages/fiche.rs:186`),
titrée « Fiche introuvable ».

Et pourtant le nœud est **rendu, coloré et cliquable** : `verset.rs:170-177` lui
donne `text-shem`, soit `--color-shem: #ba8c6c` (`style/main.css:98`), et un
`href` vers `/fr/lexique/{lemme}`.

**Le site tient donc exactement la moitié de la couche.** La couleur promet une
fiche — c'est ce que le §2.10 du vault lui fait promettre — et le clic tombe
dans le vide, 214 fois sur 215.

**Sur iOS, les mêmes nœuds fonctionnent.** `ONTTextRenderer.swift:299-305` pose
le lien `ont://shem/<lemme>`, `Router.swift:258-261` l'ouvre, et `ShemSheet` est
alimentée par `shemot.json` via `DiskShemotRepository`. Le fichier est chargé
d'un côté, pas de l'autre.

**Ce que la journée y ajoute.** Les nœuds `shem` émis dans `dist/books/` passent
de **2 734 à 2 878** — cent quarante-quatre de plus. Sur iOS ce sont cent
quarante-quatre liens neufs qui marchent ; sur le site, cent quarante-quatre
liens morts de plus.

> **La couleur et la fiche voyagent séparément, et rien ne l'annonce.**

C'est la forme que ce journal connaît déjà, un cran plus haut. Les deux dépôts
sont **cohérents avec eux-mêmes** : le site rend ce qu'il sait résoudre, l'app
résout ce qu'elle rend. La divergence ne se voit depuis aucun des deux — elle
n'existe qu'entre eux, et c'est précisément ce que ce fichier existe pour
regarder. Aucune compilation ne rougit : un index qu'on n'embarque pas ne
manque à personne.

### Et l'unique lien que le site sait résoudre ouvre la mauvaise fiche

L'entrée que les deux index ont en commun est **`moreh`**, et ce n'est pas une
coïncidence heureuse : c'est **une collision d'homographes que personne n'avait
nommée**.

    [[Moreh]]      le chêne de Moreh, où ʾAvraham s'arrête — Bereshit 12:6
    **moreh**      l'intraduisible du §2.5 — celui qui pointe du doigt la direction

Les deux tombent sur le lemme `moreh`, et il n'existe **qu'un seul fichier** :
`lexique/Moreh.md`, qui est entièrement la fiche du **concept** — *yarah*, la
racine que la *Torah* partage, la place du **moreh** parmi les cinq offices. Le
lieu n'y est pas mentionné.

Donc le lecteur qui touche le nom du chêne — sur le site **comme sur iOS** —
reçoit la leçon sur l'office d'enseignant. Le site ne rate pas 214 fiches sur
215 : il en rate 214 et **se trompe sur la quinzième**.

**Le §2.10 avait prévu le cas et n'avait pas prévu celui-là.** Il réserve à
l'auteur les homographes connus — `Shem` le fils de Noach contre `**Shem**`
l'acte d'existence, `Adam` le personnage contre `**ʾadam**` le générique — et
note que la casse ne les sépare pas. `Moreh` est **le troisième**, arrivé par la
porte de derrière : il n'est né ni d'un récit ni d'une décision, mais du jour où
`**moreh**` a été déclaré intraduisible, alors que `[[Moreh]]` existait déjà
dans un chapitre verrouillé.

> **Un intraduisible neuf peut percuter un Shem ancien, et rien ne le dit.**
> Déclarer un mot au §2.5 crée un lemme ; si `lexique/` porte déjà ce lemme pour
> un porteur, les deux fusionnent en silence. Aucun contrôle ne compare les deux
> index — et c'est exactement pourquoi celui-ci a été trouvé en mesurant leur
> intersection pour tout autre chose.

**À trancher par l'auteur**, et par lui seul : le §2.10 le dit, un homographe est
un arbitrage verset par verset.

**Ce qui reste à décider, et qui n'est pas à cette session** : embarquer
`shemot.json` et lui donner sa branche de résolution, ou cesser de colorer ce
que le site ne peut pas ouvrir. La seconde est pire pour le lecteur ; la
première est du travail dans `ONTBibleWebapp`. L'arbitrage revient à l'auteur.

### Le contrôle qui décide s'exécute au push — donc il n'a rien dit

**Trois constructions, sur trois états du vault, avec le même pipeline :**

    93c664b   la tête poussée sur origin       0 lien mort   vert
    125df42   la veille de la journée          9 liens morts, 6 lemmes
    ca8cbeb   la tête locale d'aujourd'hui     9 liens morts, 6 lemmes — la même liste

**La journée n'en a introduit aucun.** Les deux listes sont identiques à
l'octet. Les neuf viennent de la veille : `af94fdf` a posé `[[Yehoshua]]`,
`[[Yericho]]` et un `**shiphchah**` dans une glose de *Bereshit* 12 ; `e16a1f2`
a posé `[[Ashur]]`, `[[Levanon]]` et `[[Yaʿaqov]]` dans le *Sefar Gibbaraya*.

Le `**shiphchah**` mérite d'être nommé à part : la passe du 8 septembre avait
converti ce mot en `shifchah`, quarante-six fois, sur décision de règle. Le
lendemain, une glose neuve l'a réécrit **à l'ancienne graphie**, dans un
chapitre verrouillé.

> **Une passe corrige le corpus ; elle ne corrige pas la main qui écrira demain.**

**Soldé dans la journée** — `5b077df` —, et la façon dont il l'a été vaut plus
que le correctif. Le relevé que cette session a transmis donnait les quatre
occurrences **dans *Bereshit* 12**. Elles étaient deux dans *Bereshit* 12 et deux
dans *Bereshit* 17.

La faute est une lecture de colonne : le rapport de construction titre
« **Vu d'abord** », et nomme donc le **premier** fichier, jamais le seul. Le
rapport est honnête ; c'est la lecture qui a serré. Qui aurait suivi la liste
transmise aurait corrigé *Bereshit* 12, vu le compte tomber de quatre à deux, et
laissé le contrôle rouge **sans comprendre pourquoi**.

> **Une colonne qui dit « vu d'abord » ne dit pas « vu là ».** Un rapport qui
> nomme un premier exemplaire se lit comme s'il nommait un emplacement, et rien
> dans la ligne ne détrompe.

Ce n'est pas la vérification qui l'a rattrapé, c'est **le refus de croire le
relevé d'un pair** : la session du vault a refait la mesure au lieu de suivre la
liste, puis a rebalayé deux fois — vingt-trois formes hébraïques en `ph`
cherchées nommément, puis **tous** les termes en gras portant `ph`, sans liste.
Les deux rendent les mêmes quatre. C'est la règle du §2.9 tenue jusqu'au bout :
*on énumère les formes hébraïques, on ne soustrait pas les mots français.*

Le compte de liens morts est donc passé de neuf à **cinq** dans la journée. Les
cinq qui restent sont les cinq fiches de **Shemot** à écrire.

**Et le contrôle n'a pas parlé, parce qu'il ne pouvait pas.** `eprouver`
s'exécute sur pull request. Les soixante-dix commits qui portent le défaut sont
**restés en local** : le distant de la branche est encore à `93c664b`, où tout
était vert. Le dernier verdict connu de cette branche date du 8 septembre et
porte sur un état vieux de soixante-dix commits.

> **Un souvenir vert d'`origin` n'est pas l'état de `HEAD`.**

C'est la « prémisse périmée » que ce journal a déjà nommée, vue depuis
l'intégration continue plutôt que depuis un `grep` : la mesure était juste, et
elle ne dit plus rien de ce qu'on tient. Le remède est le même — construire
avant d'affirmer, et non relire un tableau vert.

### Une liste blanche protège de ce qu'elle exclut, jamais de ce qu'elle inclut

**Le fait.** La passe du 8 septembre, qui rend le het par `ch`, a converti
quatre-vingt-neuf `Haran` en `Charan`. Ils n'étaient pas le même mot :

    הָרָן   he     le fils de Terach, père de Lot
    חָרָן   het    la ville où Terach s'arrête

La passe avait été bâtie avec soin pour **ne pas** toucher *Pharaon*,
*Euphrate*, *orphelin*, *Memphis*. Elle a tenu cette promesse-là entièrement.
Personne ne lui a demandé si les formes qu'elle **contenait** étaient bien
celles qu'elle croyait.

> **Une liste blanche répond de ses exclusions. Ses inclusions, personne ne les
> relit — elles ont l'air d'être la réponse.**

Le contrôle qui manquait ne coûtait rien : regarder l'hébreu du mot avant de
changer sa translittération. C'est lui qui a tranché, sur les lemmes Strong de
`sources/he-wlc/Gen.jsonl`, qui séparent le 2039 du 2771a sans qu'on ait à en
juger.

**Séparés le 10 septembre**, et vérifié à la construction : `shemot.json` passe
de 214 à 215 entrées, `haran` s'ajoute, `charan` reste, et **ni l'un ni l'autre
n'apparaît** en « Shemot sans fiche » ni parmi les liens morts.

### Le corpus s'était mis à tirer du sens de son propre défaut

C'est la partie qui fait peur, et elle vaut plus que l'erreur elle-même.

Pendant deux jours, deux **Shem** distincts ont porté la même graphie. Les
gloses **verrouillées** de *Bereshit* 11 se sont mises à **l'expliquer** :
« les deux formes sont identiques en translittération française », « l'homonymie
n'est pas fortuite dans un texte où les **Shem** portent la destinée ». Et le
§12 du `CLAUDE.md` inscrivait « Charan personne / Charan ville (homonymie
délibérée) ».

> **Un artefact d'outil relu comme un fait du texte, et commenté comme tel dans
> un fichier verrouillé.**

Le corpus est fait pour trouver du sens ; c'est son office. Quand le défaut lui
arrive sous la forme d'une coïncidence lexicale, il fait ce qu'il sait faire, et
il le fait bien. Rien dans l'exercice ne distingue une homonymie du texte d'une
homonymie de notre translittération — sauf **regarder l'hébreu**, ce que la
glose n'avait pas à faire pour être bien écrite.

La règle qui l'interdisait était déjà écrite, une section plus haut : celle du
he final, qui refuse que `Elisha` porte à la fois אֱלִישָׁה et אֱלִישָׁע. Le même cas,
sur l'initiale au lieu de la finale — et la passe l'a produit **le jour même où
cette phrase était écrite**.

> **Une règle n'empêche que ce qu'on pense à lui soumettre.**

### Une règle syntaxique ne se vérifie pas sur la syntaxe

**Le §4.17 est né d'une erreur de la veille.** *Bereshit* 13:10 avait été aligné
sur 1:4 au motif que la construction hébraïque était identique — `וַיַּרְא` +
`אֶת` + objet plein + `כִּי`.

En reprenant les **cinq** occurrences de cette construction dans *Bereshit* :
quatre étaient déjà tranchées par des gloses verrouillées, et **deux d'entre
elles disaient l'inverse** de la règle qu'on croyait appliquer. Le regard des
fils d'**ʾElohim** sur les filles (6:2) et celui des Mitsrim sur l'**ʾIshah**
(12:14) sont rendus d'un seul verbe, délibérément : la glose de 12:14 va jusqu'à
**insister** que le verbe unique est le point.

> **Une construction identique ne demande pas un rendu identique. La syntaxe dit
> ce qui est écrit ; elle ne dit pas ce que la scène fait.**

La marche à suivre, portée au §4.17 : regarder ce qui suit le verset, regarder
qui regarde, et **regarder ce que les gloses du corpus disent déjà** — elles
portent souvent l'arbitrage, écrit avant que la règle ait été formulée.

Et le §4.17 va plus loin que le verbe *raʾah*, sur décision de l'auteur : *on ne
fait pas des mathématiques linguistiques ; le vivant peut être singulier.* Une
restitution sert le sens, un système se sert lui-même — quand les deux
divergent, c'est le sens qui commande.

### Une garde peut casser le contrôle qu'elle définit

`sessions/` est exclu des balayages du dépôt : ce sont les transcriptions de ce
que l'auteur a tapé, et corriger l'orthographe de ce que quelqu'un a écrit n'est
pas une correction, c'est une réécriture. La journée a posé un `sessions/README.md`
pour que cette garde soit lisible plutôt que sue.

Le `README` portait un témoin de comptage — « il doit rester exactement *n*
occurrences de tel mot dans ce dossier » — et **citait ce mot deux fois dans sa
propre page**. Le compte est passé de *n* à *n+2* le jour où la garde a été
écrite.

> **Une page qui définit un compte fait partie de ce qui est compté.**

Réparé dans la journée, en coupant le mot dans la citation : le dossier rend de
nouveau le compte attendu, et la page le dit d'elle-même.

Le motif n'est pas anecdotique : c'est celui du §13.2 et celui du périmètre du
§2.5 — un document qui énonce une règle est lui-même dans le champ de la règle,
et personne ne pense à s'y regarder.

### Un zéro est vrai pour ce que l'instrument cherche, faux pour ce qu'il prétend mesurer

La passe de casse sur `**shem**` / `**Shem**` (§2.5 : majuscule après
déterminant) a corrigé trente-six occurrences dans neuf fichiers, puis annoncé
« reste après déterminant : 0 ». Il en restait **neuf**.

Le motif énumérait les déterminants **en minuscules seulement** —
`le|son|leur|du|des|ce|un` —, si bien que « Le **shem** » en tête de phrase et
« Leur **shem** » après un point ne matchaient pas. **Cinq des neuf étaient dans
le seul *Bereshit* 17**, le fichier où la même passe en avait corrigé dix.

Le zéro était exact pour ce que l'instrument cherchait, faux pour ce qu'il
prétendait mesurer.

> **Un zéro se rapporte avec son outil.** Et un zéro est ce qu'on vérifie le
> moins, parce qu'il ne laisse aucun appariement à relire — c'est l'endroit
> exact où un instrument mal réglé passe inaperçu.

**Et ce n'est pas un contrôle qui l'a trouvé** : c'est la session qui travaillait
dans *Bereshit* 17 et qui l'a vue en passant. Aucun relevé n'allait la chercher,
puisque le relevé disait zéro.

Trois autres divergences remontées le même jour ont dû être **rejetées** pour la
raison inverse, et elle mérite d'être gardée : `**Nephilim**`, `**ʿirin**` et une
prétendue contradiction du §2.9 étaient toutes trois des faux positifs, lus dans
les passages du `CLAUDE.md` qui **racontent une graphie abandonnée**. Un document
qui garde l'histoire de ses règles offre à qui le parcourt trois graphies mortes
pour une vivante, et rien dans la ligne ne dit laquelle est laquelle.

> **Prémisse périmée non pas dans le dépôt, mais dans la phrase lue.**

### « Attesté » compte le gras, pas ce qui s'atteint

Un chiffre du rapport a bougé sans que rien ne se perde, et il vaut d'être
expliqué avant que quelqu'un ne le lise comme une régression :

    Entrées attestées dans le corpus rédigé    71 → 70

L'entrée qui sort est `tevah`. La journée a retiré son unique gras — `**tevah**`
dans *Sefar Gibbaraya* 10:17 — et le retrait est **juste** : `tevah` est un
terme du §3.2, rendu « arche », pas un intraduisible du §2.5, et le gras y était
un défaut.

Mais la fiche reste atteignable : `dist/books/bereshit.json` porte toujours
**quatre** nœuds `translit` dont la `cible` est `term/tevah`, posés par les
niveaux 3 `(*tevah* / תֵּבָה)`. Aucun lecteur ne perd l'accès.

> **Le compte des « attestées » mesure le gras, non la portée.** Deux questions
> voisines, un seul mot pour les dire — et c'est le genre d'écart qui a déjà
> laissé passer un défaut le 9 septembre, quand le rapport normalisait autrement
> que le consommateur.

### Ce que ça demande à chaque dépôt

- **`ONTBibleTranslation`** — **cinq** fiches de **Shemot** à écrire avant que la
  branche puisse être poussée sans rougir `eprouver` : `Ashur`, `Levanon`,
  `Yaʿaqov`, `Yehoshua`, `Yericho`. Le sixième lemme, le `**shiphchah**` des
  *Bereshit* 12 et 17, a été soldé dans la journée. C'est de l'écriture de
  corpus : elle revient à la session qui tient le vault, pas à celle qui mesure.
- **`ONTBibleApp`** — rien à porter. Le §4.17 n'a produit aucun lemme, vérifié
  dans `pipeline/src/reference.rs` : `read_tagged_terms` ne s'exécute que sur la
  section numérotée `2.5`, `read_fixed_terms` sur `^3\.[123]$`, et le motif des
  formes exige des **accents graves** autour du gras — un `**terme**` de prose
  n'est jamais lu. `glossary.json` reste à 139 entrées avant et après. La couche
  des **Shemot** y est complète, couleur et fiche.
- **`ONTBibleWebapp`** — la moitié de la couche des **Shemot** est à finir, et
  c'est le seul vrai chantier ouvert par la journée. Voir plus haut.

### Deux dettes constatées en passant, et non traitées ici

- **Les quatre `SYNCHRONISATION.md` divergent**, et divergeaient avant cette
  entrée : `concorder-la-synchronisation.py` rend trois empreintes de tronc
  commun pour trois dépôts, donc **aucune référence**. La PR #83 du vault porte
  déjà les entrées manquantes de l'app ; elle n'est pas fusionnée, et le site
  n'a pas son équivalent. Cette entrée est ajoutée identique aux trois — elle ne
  réduit pas l'écart préexistant, et il ne fallait pas qu'elle le fasse en
  silence.
- **Le nœud `renvoi` de la quatrième couche est déjà rendu par les deux
  liseuses**, alors que le §2.11 demande de ne pas encore en écrire. Le site
  l'affiche coloré et **assume** de le laisser inerte, commentaire à l'appui.
  iOS lui pose un lien `ont://chuqqah/<cible>` que **`Router.swift` ne route
  pas** : le `switch` couvre `term`, `shem`, `verse`, `share`, `read`, puis
  `default: return false`. Le jour où une **chuqqah** portera un `((…))`, iOS
  soulignera un lien mort là où le site aura été honnête. Rien ne le signale
  aujourd'hui, puisque le vault n'en écrit aucun.
