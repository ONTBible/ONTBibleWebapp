# ONTBibleWebapp — le site de La Bible ONT

Document de reprise. Il contient ce qu'une nouvelle session ne peut pas
deviner : les décisions déjà prises, leurs raisons, et ce qui reste ouvert.

---

## 1. Ce qu'est ce site, et pour qui

`ontbible.com` — le site public de **La Bible ONT**, une restitution française
du corpus hébreu et araméen antique fondée sur l'ontologie hébraïque
fonctionnelle.

Il répond à trois besoins, dans cet ordre d'importance :

1. **Dire le pourquoi.** Aujourd'hui, un lien partagé depuis l'app arrive sur
   une page qui ne dit rien. Quelqu'un qui découvre l'ONT n'a nulle part où
   comprendre de quoi il s'agit.
2. **Servir les liens partagés** — `ontbible.com/fr/lire/<livre>/<unité>?v=1-3`
   et le fichier d'association des liens universels.
3. **Fournir ce que l'App Store réclamera** : une politique de confidentialité,
   des conditions, une adresse marketing.

## 2. L'auteur — à lire avant d'écrire une ligne

**Gloire Bikouta.** « Gloire » est son deuxième prénom, celui qu'il emploie en
public, y compris sur GitHub. **Jamais « Sha'eliel »** : c'est son nom
fonctionnel *interne* au projet, présent dans le vault, et il ne sort pas.

**Son registre.** Il écrit en parataxe — « Et… Et… Et… », le rythme du waw
consécutif hébreu. Phrases courtes, verbales, enchaînées sans subordination.
Aucune atténuation. Et il laisse les termes hébreux **debout, sans glose** :
*sod*, *neshamah*, *Olam*, *Maqom*, *Ish*, *bene ha-adamah*.

Ce n'est pas une méthode qu'il a adoptée pour l'ONT : c'est déjà sa façon
d'écrire en privé depuis des années. **Le site doit sonner comme lui**, pas
comme une notice. Phrases courtes et affirmatives, mots hébreux laissés tels
quels. Pas de « en revanche », pas de « il convient de noter ».

**L'origine du projet est publique — vision comprise.** Décision prise le
12 août 2026, après lui avoir proposé trois positions (rien de personnel /
l'origine nommée sans la vision / la vision). Il a choisi la troisième.

La source est une page Notion **privée** — *The Holy Day*, base « Book of the
Torpor » — écrite le 27 juillet 2026 sur une vision reçue deux ans plus tôt.
Elle est verrouillée, et il y écrit qu'il avait cru qu'elle ne devait pas être
écrite. **En tirer une version publique, pas la recopier.** Le cœur :

> un manteau d'antiquité, et l'ordre de ramener ce qui a été perdu depuis
> d'anciens temps — « les choses dont le monde n'a même pas conscience qu'il
> ne sait pas ».

Lui demander le texte définitif de cette page. Proposer un premier jet plutôt
qu'une page blanche, mais **ne pas publier sans sa relecture**.

## 3. La pile

**Leptos 0.8 en mode SSR + Axum, et Tailwind v4.** Tout en Rust, rendu côté
serveur — sa demande explicite. `cargo-leptos` télécharge lui-même le binaire
Tailwind ; il n'y a ni `node_modules`, ni `tailwind.config.js` (la v4 se
configure en CSS, dans `style/main.css`).

**L'architecture est en couches, et une flèche ne remonte jamais :**

```
domaine        ──▶  rien                       pur, compile aussi en wasm
application    ──▶  domaine                    déclare des ports (traits)
infrastructure ──▶  application, domaine       les réalise — `ssr` seulement
interface      ──▶  application, domaine       design/ (forme) + pages/ (propos)
main.rs        ──▶  tout                       la racine de composition
```

Le domaine ne prend même pas de bibliothèque de dates : il reçoit un numéro de
jour. C'est ce qui le rend testable sans horloge, et identique des deux côtés.

**Un composant par fichier**, `design/` pour la forme et `pages/` pour le
propos. Une page n'écrit aucune valeur de style — si une forme manque, elle se
crée dans `design/`. Depuis Tailwind, la forme vit **dans** le `.rs`, à côté du
balisage : il n'y a plus de feuille parallèle qui puisse diverger.

Le backend de l'app est déjà du Rust sur AWS Lambda (`ONTBibleApp/backend/`,
axum + `lambda_http`, architecture hexagonale, eu-west-3). Le site suit la
même voie : `cargo lambda`, même région, même façon de déployer. C'est
cohérent, et ça évite d'introduire un hébergeur de plus.

**Réserve à connaître** : un démarrage à froid Lambda mesuré à ~450 ms. Sur un
site vitrine, c'est visible. Si ça gêne, les options sont une réservation de
concurrence provisionnée (payante) ou un hébergeur qui garde le processus
chaud (Fly.io, Railway). À trancher après l'avoir vu tourner, pas avant.

**Cloudflare Pages est écarté** : il ne fait pas tourner de serveur Rust.

## 4. L'architecture des domaines

**Fait le 13 août 2026.** Le site tient la racine.

| | |
|---|---|
| `ontbible.com` | **le site** — CloudFront → API Gateway → Lambda |
| `www.ontbible.com` | 301 vers `ontbible.com` |
| `labibleont.com` | 301 vers `ontbible.com` |
| `www.labibleont.com` | 301 vers `ontbible.com` |
| `api.ontbible.com` | **l'API de l'app** — domaine régional d'API Gateway |

Tout en **nuage gris** chez Cloudflare. Le proxy orange présente le certificat
de Cloudflare devant celui d'AWS : le nom ne correspond plus, et le TLS casse.

Les trois renvois passent par une fonction CloudFront qui préserve **chemin et
paramètres** : un lien vers `labibleont.com/fr/lire/bereshit/bereshit-1?v=1-3`
arrive sur ces versets, pas sur l'accueil.

### L'ordre de migration du §4 reposait sur une hypothèse fausse

Il disait : basculer `ONTAPIBaseURL` dans l'app **avant** de donner la racine au
site, sinon les liens universels tombent. Vérifié dans `app/project.yml` :

```
ONTAPIBaseURL : https://j451hq8d3k.execute-api.eu-west-3.amazonaws.com
ONTWebBaseURL : https://ontbible.com
```

L'app appelle son API **directement sur `execute-api`**. `ontbible.com` ne lui
sert qu'à *fabriquer* les liens partagés et à les reconnaître au retour. Ce
domaine ne servait donc que deux choses — le fichier d'association et la page de
repli d'un passage — que le site rend toutes les deux, en mieux.

La seule condition réelle était que l'association reste **identique et
ininterrompue**. Elle l'est, à l'octet près : même `appIDs`, même `components`,
même `application/json`.

`api.ontbible.com` reste utile — une adresse stable, indépendante de
l'identifiant `execute-api` qu'AWS change si l'API est recréée — mais ce n'était
pas un prérequis.

### Deux routes doivent rester servies à la racine

- `/.well-known/apple-app-site-association` — **`application/json`, aucune
  redirection**. iOS ne le relit qu'à l'installation et Apple le met en cache
  via son propre CDN : une erreur ici ne se voit pas tout de suite.
- `/fr/lire/{livre}/{unité}` — la page d'un passage, avec ses balises Open
  Graph. Le segment de langue est délibéré : il épargne une migration le jour
  d'une édition anglaise.

Les deux sont servies par le site, et vérifiées en ligne.

### Ce qu'a coûté la bascule

Dix minutes de panne, causées par une valeur de validation collée dans
l'enregistrement `ontbible.com` lui-même. Le domaine ne résolvait plus du tout.

La leçon tient en une phrase : **une ligne dont le nom commence par `_` reçoit
une valeur qui commence par `_`**. Les enregistrements de validation ne portent
aucun trafic — les casser ne casse rien de vivant. Celui du domaine, si.

Le retour en arrière reste possible : le domaine personnalisé de l'API n'a pas
été détruit. Remettre `d-xdeopzr27e.execute-api.eu-west-3.amazonaws.com` sur
`ontbible.com` rend l'état d'avant en une minute.

## 5. La direction artistique

**« Un peu ancienne »** — sa demande. Le livre imprimé classique, pas le
pastiche : filets fins, capitales espacées, larges marges, papier plutôt
qu'écran.

### Le site est une nuit d'aubergine

Trois essais ont été nécessaires, et les deux premiers sont instructifs. Le
site a d'abord suivi le thème du système : chez un lecteur en mode sombre, il
retombait sur un gris quasi noir, sans matière — une page quelconque. On l'a
alors interdit et posé le site sur du parchemin. Les deux manquaient la cible :
**il le voulait sombre**, mais une aubergine, pas un noir.

La rampe est **dérivée de la marque** — teinte constante 343°, celle de
`#421B26` relevée au pixel, seule la clarté varie. Les valeurs sont calculées,
pas choisies à l'œil ; la marque elle-même retombe sur `#411B26`, ce qui
vérifie la dérivation.

| jeton | valeur | rôle |
|---|---|---|
| `nuit` | `#18090D` | le fond de la page |
| `surface` | `#261016` | une carte, une porte, une ligne |
| `surface-haute` | `#35151E` | un bandeau, ce qu'on détache |
| `aubergine` | `#421B26` | la marque, le haut des dégradés |
| `or` / `accent` | `#CDBE83` | l'intraduisible, les titres de section, les filets |
| `encre` | `#CFC5B9` | le corps — 11,4:1 |
| `encre-vive` | `#EDE3D6` | un titre — 15,3:1 |
| `encre-douce` | `#9D948B` | le niveau 2 — 6,5:1 |
| `important` | `#D87994` | le terme important — 6,5:1 |

**Jamais de blanc pur**, et jamais un gris. La halation fait déborder un texte
clair dans le noir qui l'entoure : les lettres paraissent plus grasses, vibrent
et se doublent sur une dalle OLED. Descendre à 87 % supprime l'essentiel du
halo sans rien coûter — on reste bien au-delà du seuil AAA. Et l'encre garde la
chaleur du parchemin de l'app plutôt que de virer au gris de laboratoire.

Trois conséquences dans `style/main.css`, et ce ne sont pas des goûts :

- `-webkit-font-smoothing: antialiased`. Il amincit les traits — sur fond clair
  il les fait disparaître, sur fond sombre il compense exactement le
  débordement optique. C'est le seul cas où il faut le forcer.
- un souffle d'interlettrage (`0.006em`) : sur une nuit, les lettres claires se
  rapprochent visuellement et finissent par se toucher.
- `grain-page` — un bruit `feTurbulence` à 3,5 %. **Nécessité technique** : un
  dégradé sombre étalé sur une page se découpe en bandes, parce qu'un écran
  n'a que 256 valeurs par canal et que l'écart entre deux nuances de nuit est
  plus petit que ça. Le bruit casse les bandes. Il donne au passage la matière
  d'un papier ancien.

Les filets sont **dorés à 18 %**, pas gris : sur une nuit chaude, un gris
paraît sale.

`voile-aubergine` donne la profondeur d'un bandeau — une lueur haute, la nuit
qui reprend vers le bas, de sorte que la lumière semble venir d'au-dessus du
texte, comme dans une nef. Trois couches et pas une de plus : au-delà, un
dégradé cesse d'être une atmosphère et devient un effet.

### Typographie

| rôle | fonte |
|---|---|
| titres, navigation, capitales | **Jost** — la géométrique du combination mark |
| tout le reste, citations comprises | **Literata** — celle du corps de l'app |
| hébreu | **Ezra SIL** — la seule qui positionne niqqud et te'amim |

**EB Garamond a été essayée puis retirée**, et il faut savoir pourquoi avant de
la reproposer : c'est la lettre du livre imprimé classique, mais ses déliés
très fins bavent sur fond sombre et ses empattements vibrent. C'est le défaut
connu des romanes à fort contraste en mode sombre. Literata est dessinée pour
l'écran, à faible contraste de graisse — et c'est déjà la fonte du corps de
l'app, donc un verset cité ici se lit exactement comme sur le téléphone.

Jost ne tient pas dix lignes de texte suivi : hauteur d'x basse, fûts égaux.
Elle garde les titres, où sa géométrie fait écho à la marque.

### Le portrait — pas de cadre, et il sort de la colonne

Deux erreurs successives, et la seconde était plus subtile que la première.

La photo flottait à même la page, détourée : elle avait l'air d'un autocollant,
et la chemise blanche faisait une masse lumineuse. On l'a donc inscrite dans une
arche bordée d'un filet d'or — ce qui l'a **enfermée dans une boîte**. Le
rectangle se voyait plus que le sujet.

Ce qui remplace le cadre est une **lueur** : un halo d'aubergine flou derrière
les épaules, sans contour. Il fait le même travail — donner un fond au sujet
pour qu'il ne flotte pas — sans rien dessiner. On ne voit pas d'où vient la
lumière, seulement qu'il y en a une. Le bas continue de se dissoudre dans la
nuit (`fondu-bas`) : le détourage laisse le vêtement s'effilocher, autant en
faire une intention.

Et le portrait **sort de la colonne de lecture** par la gauche
(`lg:-ml-40 lg:w-80`). La mesure borne le *texte* ; elle n'a aucune raison de
borner une image, et la marge du site est vide. Il y gagne le tiers de taille
qui lui manquait sans que la ligne de texte s'allonge d'un signe. En dessous de
`lg` il n'y a plus de marge à occuper : il repasse dans la colonne, puis
au-dessus du texte sur téléphone.

C'est le patron à réutiliser pour toute image du site.

### L'échelle typographique

Le corps est à **21 px**, pas à 16. Deux effets se cumulent : Literata a une
hauteur d'x généreuse mais une **chasse étroite**, et sur une nuit d'aubergine
le texte clair **paraît plus petit** qu'il ne l'est — l'inverse de la halation,
qui l'épaissit sans l'agrandir. Les essais à 17 puis 19 px se lisaient encore
comme des notes de bas de page. Il a demandé plus grand deux fois : ne pas
redescendre sans qu'il le demande.

| | |
|---|---|
| `text-sm` | 16 px — capitales espacées, légendes |
| `text-base` | **21 px** — le corps |
| `text-lg` → `text-3xl` | quarte juste, en `clamp` |

**La mesure suit la taille**, et c'est la règle à retenir : à 21 px, 34 rem ne
tiendraient plus que 52 signes par ligne — sous la fourchette confortable de 55
à 75, et le texte se hacherait. Elle passe donc à 38 rem, soit environ 58
signes : toujours du côté étroit qu'il préfère. Grandir le texte sans élargir
la mesure produit des lignes trop courtes, ce qui se lit aussi mal que des
lignes trop longues.

L'interligne descend à 1,68 en compensation : plus le corps grandit, moins il a
besoin d'air proportionnel pour que l'œil retrouve la ligne suivante.

### L'ouverture est **une** unité

**Toutes** les ouvertures remplissent l'écran, l'accueil comme les pages
intérieures. La variante `sobre` ne diffère que par la **lumière** — jamais par
la hauteur. Un essai l'avait fixée à 70 % : les pages intérieures montraient
alors leur ouverture *et* le début du bloc suivant, ce qui détruit exactement
l'unité recherchée.

Et le contenu y est **centré dans la hauteur restante**, sur les quatre pages.
Un premier écran se lit comme une affiche : le vide au-dessus vaut celui d'en
dessous, et un contenu posé en haut laisse en bas un trou qu'aucun regard ne
comble.

**Décidé le 13 août 2026**, et c'est un revirement — l'ancrage en haut, à
distance fixe, était la règle d'avant. Elle avait sa raison : le rappel et le
titre tombaient exactement à la même hauteur d'une page à l'autre, vérifié au
pixel à y = 328, là où le centrage fait commencer plus haut une ouverture dont
le contenu est plus haut — la page de l'auteur, qui porte un portrait, en
gagnait cent soixante.

Le prix est connu et assumé. Ce qui le rend acceptable : toutes les ouvertures
remplissent l'écran, donc toutes se centrent de la même façon, et l'unité se
lit dans la règle plutôt que dans une coordonnée. Ne pas revenir à l'ancrage
sans qu'il le demande.


Ce qui frappe en arrivant sur un site bien fait, c'est un premier écran qui est
une seule chose et qui le remplit. Le premier essai posait l'en-tête en bande
au-dessus de l'ouverture : on voyait alors deux objets — un bandeau, puis un
écran — et l'effet tombait.

**`Hero` contient l'en-tête.** La marque et la navigation flottent dans le lieu
au lieu de le surmonter — d'où le `z-20` et l'absence de fond.

Un premier montage remontait l'ouverture *sous* l'en-tête, d'une marge négative
égale à un jeton `--hauteur-entete`. **Ça ne pouvait pas tenir** : sur un
téléphone, la navigation passe sur plusieurs lignes et l'en-tête devient deux
fois plus haut que le nombre inscrit. La bande réapparaissait, et le contenu
débordait sous la ligne de flottaison. Un nombre magique qui décrit la
géométrie d'un autre élément est toujours faux quelque part.

Les pages sans ouverture — les légales, l'erreur — posent leur en-tête
elles-mêmes. Il n'y en a jamais deux : vérifié, une balise `<header>` par page.

La navigation se resserre en dessous du seuil : à 16 px avec un interlettrage
de 0,16 em, ses trois entrées passaient sur trois lignes sur un téléphone. Elle
en porte **cinq** depuis que la liseuse existe — « Lire » et « Lexique » en
tête, le corpus avant le discours sur le corpus — et tient sur deux lignes.
Le seuil n'est pas le nombre d'entrées mais la hauteur qu'elles prennent :
à vérifier au simulateur à chaque ajout, jamais à l'œil sur un grand écran.

### Deux classes Tailwind sur la même propriété : c'est la feuille qui tranche

`Bloc` portait `max-w-mesure` en dur **plus** `max-w-large` en conditionnel. Les
deux se retrouvaient sur l'élément, à spécificité égale — et à spécificité
égale, c'est l'ordre de la **feuille de style** qui décide, jamais celui de
l'attribut. Tailwind range `.max-w-mesure` après `.max-w-large`.

Le prop `large` ne faisait donc **rien**, nulle part, depuis le premier jour. La
comparaison de l'accueil — la pièce qui porte tout le site — se composait sur
38 rem au lieu de 52 : deux colonnes de 185 et 343 px, où le texte se césurait à
chaque ligne (« com-mença », « exis-tence »). Corrigé, elles font 263 et 489 px,
et la césure disparaît sans qu'on ait touché à la typographie.

Un défaut de ce genre est invisible : la page ne casse pas, elle est seulement
étroite, et rien ne dit qu'elle devrait l'être moins. **La règle** : deux
valeurs d'une même propriété doivent être exclusives dans le balisage, pas
seulement dans l'intention.

```rust
class="mx-auto w-full px-6"           // rien qui touche à max-width
class=("max-w-mesure", !large)
class=("max-w-large", large)
```

Le même piège dormait dans `blocs.rs` (`px-4` et `ps-5`) : il marchait, mais par
l'ordre de la feuille, donc jusqu'au jour où Tailwind rangerait autrement.

### Un écran par bloc

Chaque `Bloc` occupe au moins la hauteur de la fenêtre, contenu centré. Sans
ça, les fonds clairs et sombres épousaient la longueur des textes et se
lisaient comme des rayures posées au hasard.

`min-h-dvh`, et les deux mots comptent :

- **minimale** et non fixe — la comparaison et les pages légales dépassent un
  écran, une hauteur fixe les couperait ;
- **`dvh`** et non `vh` — sur un téléphone, `vh` compte la fenêtre sans la
  barre d'adresse, qui se rétracte au défilement : chaque bloc sauterait de
  quelques dizaines de pixels au premier geste.

L'ouverture, elle, occupe l'écran **moins l'en-tête** (`--hauteur-entete`) :
sinon le premier écran montrerait la marque, puis le haut d'un hero d'un écran
entier, et la phrase tomberait sous la ligne de flottaison.

### « Ancienne mais moderne » — où ça se joue

L'ancien donne la **structure**, le moderne donne la **retenue**. Chaque fois
qu'un signe ancien est employé, il l'est sans ornementation : c'est ce contraste
qui empêche la page de virer à l'antiquaire.

| le signe, ancien | l'exécution, moderne |
|---|---|
| chiffres romains sur les sections (`TitreDeSection`) | un chiffre, un filet court, beaucoup d'air — ni cartouche ni enluminure |
| lettrine sur trois lignes (`lettrine`) | une lettre d'or dans la fonte des titres, sans fond ni cadre |
| chiffres elzéviriens dans le texte | et chiffres alignés de chasse fixe dans les tableaux — une colonne n'est pas une phrase |
| ponctuation suspendue | `hanging-punctuation`, que le navigateur fait seul |
| portrait en arche, comme une gravure | un filet d'or d'un pixel, une surface unie, un recadrage net |

Deux pièges rencontrés, à ne pas refaire :

- `::first-letter` **embarque la ponctuation qui précède**. Un paragraphe qui
  commence par un guillemet met donc le guillemet dans la lettrine — et la
  ponctuation suspendue le jette dans la marge par-dessus. Les paragraphes à
  lettrine commencent par une lettre.
- Un filigrane de montagne trop grand et rogné par son cadre ne se lit plus
  comme une montagne : c'est du bruit. Il n'en reste qu'un, dans le `Hero`, à
  6 % et débordant par le haut de sorte qu'on lise une crête.

### Les trois niveaux du texte — le cœur du projet

Si le site affiche du corpus, il doit les rendre **distincts**. C'est la
raison d'être de tout le pipeline (voir `ONTBibleApp/README.md`) :

| dans le `.md` | rôle | rendu |
|---|---|---|
| texte nu | niveau 1 — le corps | encre |
| `**mot**` | intraduisible | **or**, cliquable vers sa fiche |
| `==mot==` | terme important | **`#862742`**, semi-gras, **inerte** |
| `*[glose]*` | niveau 2 | plus petit, encre atténuée |
| `(*translit* / hébreu)` | niveau 3 | italique + Ezra SIL, isolation bidi FSI/PDI |

L'or **promet** une fiche et la tient. Le bordeaux clair marque sans rien
promettre. Ne pas les confondre.

## 6. Les ressources

Dans `public/images/` — et pas dans un `assets/` séparé : `public/` est le
dossier que `cargo-leptos` recopie tel quel vers la racine du site, donc une
image y est à la fois la source et le livrable. Un second dossier obligerait à
synchroniser les deux, et l'un des deux finirait périmé.

| fichier | usage |
|---|---|
| `logomark.svg` | la montagne seule — **favicon**, et signe de section |
| `wordmark.svg` | « La Bible ONT » et מקרא הקדם |
| `combination-mark.svg` | les deux, séparés par un filet |
| `montagne-512.png` | repli de favicon, pour les navigateurs sans SVG |
| `portrait-640.webp`, `portrait-1024.webp` | Gloire Bikouta, détouré depuis le brut |
| `apercu.png` | l'aperçu des messageries, 1200 × 630 — composé, voir ci-dessous |
| `wordmark-avec-r.svg`, `combination-mark-avec-r.svg` | les originaux **avec le ®**, gardés pour le jour de l'enregistrement |
| `touch-icon.png` | l'icône d'écran d'accueil iOS, opaque |
| `app-store-fr.svg` | le badge officiel d'Apple, en français et en blanc |
| `qr-app.svg` | le QR vers la fiche — `scripts/qr-app.py` |
| `app-lecture.webp` | une capture de l'app, prise dans `ONTBibleApp/app/Captures/` |

Les SVG sortent d'Affinity et passent par `scripts/normaliser-svg.py`, **puis
par `scripts/retirer-le-r.py`** — tous deux idempotents, à relancer après chaque
nouvel export, sans réfléchir. Le second retire le ® tant que la marque n'est
pas enregistrée (§9), et il le trouve par la géométrie plutôt que par un numéro
de sous-tracé, qui changerait au prochain export : c'est le plus petit groupe de
contours, pris depuis la droite, qui ne chevauche plus rien — les filets mis à
part, puisque celui du combination mark traverse toute la largeur. Il pose la
`viewBox` que l'export omet — sans elle un vecteur ne se met pas à l'échelle,
ce qui lui retire sa raison d'être — retire la taille en pixels pour que la CSS
décide, et remplace la couleur en dur par `currentColor`, plus l'or de la
marque en attribut de présentation : une valeur par défaut qu'une règle CSS
écrase, donc la montagne suit le thème sombre tout en restant dorée quand le
fichier est ouvert seul.

L'or exporté par Affinity vaut `#CFBD7A`, pas `#CDBE83` — un décalage de profil
colorimétrique. C'est le second qui fait foi : il est relevé au pixel sur le
rendu de la marque.

`scripts/images-sociales.py` compose l'aperçu et l'icône **depuis les jetons du
site** plutôt qu'à la main : une image d'aperçu dessinée à part trahit le jour
où la marque bouge. Deux raisons de les avoir faites :

- l'aperçu servait la montagne seule en 512 × 512, un carré dans un cadre
  paysage — la plupart des messageries le rognent ou l'entourent de blanc ;
- iOS ne gère pas la transparence d'une icône d'écran d'accueil : il la remplit
  de noir. Il faut un fond d'aubergine opaque.

Le script rastérise les SVG par QuickLook, qui rend sur **fond blanc opaque**.
La transparence est reconstruite par l'arithmétique — le tracé n'a qu'une
couleur, donc α = (255 − L) / (255 − L_or). Un détourage naïf récupérerait le
carré entier, ce qu'un premier essai a fait.

Le portrait est détouré **depuis le fichier brut** — `~/Downloads/IMG_3655.DNG`,
5348 × 7132 — par `scripts/portrait.py`. Ce fichier n'est pas dans le dépôt :
gardez-le, c'est la seule source. `Doneground/CV/done/photo CV/export/pro.png`
est un ancien détourage sur fond blanc, désormais inutile.

Le script mesure au lieu de supposer. Le mur n'est pas uniforme — 202 de
luminance à hauteur de tête, 142 à hauteur d'épaule — donc il est **modélisé**
ligne par ligne depuis les deux marges. Les seuils viennent de mesures : le
résidu du modèle sur le mur plafonne à 39, l'écart sur le visage commence à
113 ; on prend 42 et 95, et les mèches vivent entre les deux.

Et l'opacité se calcule **en cherchant le fond, pas le sujet**. Seuiller sur
« ressemble au sujet » perce un trou dans la chemise blanche là où un pli a la
valeur du mur. Le mur, lui, est connexe au bord de l'image — ce qu'aucune
partie du sujet n'est. On propage donc depuis les bords, et tout ce qui n'est
pas atteint appartient au sujet.

Le cadrage s'arrête au buste : plus bas, l'ombre creuse sous l'épaule gauche se
lit comme une déchirure sur fond sombre. Elle est dans la photographie, pas
dans le détourage.

Sortie en **WebP à qualité 85** : 152 Ko contre 1 665 en PNG, onze fois moins,
sans différence visible et en gardant l'alpha. Pas de repli PNG — tous les
navigateurs lisent le WebP depuis 2020, et un repli qu'on ne teste jamais est
un fichier mort.

La montagne existe aussi en catalogue d'assets 1×/2×/3× dans
`ONTBibleApp/app/Packages/ONTDesignSystem/Sources/ONTDesignSystem/Resources/`.
Lui **demander un export SVG** : il l'a proposé, c'est le bon original pour un
usage en grand sur le web.

## 7. Les pages, et où trouver la matière

Le vault est le dépôt voisin : `../ONTBibleTranslation/`.

| page | contenu | source |
|---|---|---|
| **Accueil** | le principe en une phrase — *le cosmos hébreu n'est pas une usine, c'est un Temple* — et le verset du jour | `CLAUDE.md` §1 |
| **Le pourquoi** | l'ontologie fonctionnelle ; pourquoi *bara* ne veut pas dire fabriquer ; les trois niveaux montrés sur un vrai verset | `CLAUDE.md` §1, §2.1 |
| **Ce que l'ONT n'est pas** | ses cinq lignes, telles quelles — la page la plus forte, la seule qui prend un risque | `CLAUDE.md` §10 |
| **L'auteur** | l'origine, la vision, le rapport de travail avec Claude — qu'il assume | `context/auteur.md` + sa relecture |
| **L'app** | la capture, le badge App Store, le QR — et Android dit franchement absent | `ONTBibleApp/app/Captures/` |
| **Lire** | les passages partagés | `ONTBibleApp/dist/` |
| **Confidentialité / Conditions** | à rédiger — comptes, synchronisation, Sentry, RGPD | — |

**Le verset du jour est fait, et l'accord avec l'app est prouvé.**

C'est une *fonction de la date*, pas un tirage : l'app, le widget et la
notification tombent sur le même verset le même jour sans se parler. Le portage
vit dans `domaine/verset_du_jour.rs`, le vivier est embarqué à la compilation
depuis `../ONTBibleApp/dist/daily.json` (251 versets) — **jamais copié**, sinon
la règle éditoriale du pipeline et celle du site divergeraient.

Deux pièges découverts en le faisant, et tous deux sont maintenant tenus par
des témoins relevés en **exécutant le Swift** :

1. **La permutation.** Un pas fixe premier avec la taille du vivier, valant
   ~0,618 × celle-ci. Il garantit qu'aucun verset ne revient avant que tous
   soient passés — un tirage donnerait un doublon dans le mois avec quatre
   chances sur cinq.

2. **Le décalage d'un jour.** L'app fait `startOfDay` (minuit **local**) puis
   divise cet horodatage comme s'il était UTC. Pour tout fuseau à l'est de
   Greenwich, le numéro de jour vaut donc *la date civile moins un*. Le verset
   change bien à minuit chez le lecteur — seul l'entier est décalé — mais comme
   le choix est une permutation de cet entier, un jour d'écart donne **un autre
   verset**. `infrastructure/horloge.rs` reproduit le calcul de l'app ; le
   corriger ferait diverger le site du téléphone du même lecteur.

   Le premier témoin employait un calendrier en UTC : il passait, et le site
   annonçait quand même un autre verset. C'est le fuseau de l'édition qu'il
   fallait éprouver.

## 7 bis. Voir le site

Deux outils, et ils ne servent pas à la même chose.

`scripts/apercu.py` rend une page avec QuickLook, qui embarque WebKit. Il suffit
à juger une composition sur grand écran.

`scripts/sim.sh` ouvre une page dans le **simulateur iOS** et en capture
l'écran. C'est la seule vérification qui vaille pour tout ce qui dépend de la
largeur : QuickLook rend à une fenêtre fixe puis réduit l'image, donc les
requêtes média y voient toujours un grand écran. Toutes les vérifications
« mobile » faites avec lui étaient sans valeur, et une bande qui réapparaissait
sur téléphone est passée à travers.

    ./scripts/sim.sh /fr/l-auteur

Le simulateur partage le réseau de l'hôte : `127.0.0.1:3000` lui répond. Le
script prend l'appareil nommé « Web » s'il existe — les autres portent l'app
ONT, et y ouvrir Safari les sortirait de leur état.

**Une limite connue** : QuickLook ne rend pas les masques CSS pointant un SVG
externe. Le signe de section et les massifs y apparaissent comme des vides.
Ce n'est pas un défaut de la page — ne pas « corriger » ces blancs.

## 8. Où en est le site

**Fait** — squelette Leptos SSR, couches, design system, verset du jour accordé
à l'app, vecteurs de la marque normalisés, métadonnées complètes (canonique,
hreflang, Open Graph, JSON-LD), **et la liseuse complète** (voir §8 bis).

```
/            → 307 vers /fr        (temporaire : « / » choisira la langue un jour)
/fr                                la page d'accueil — voir ci-dessous
/fr/le-pourquoi                    l'essai de fond
/fr/l-app                          installer l'application
/fr/ce-que-l-ont-n-est-pas         les cinq lignes du §10 du vault
/fr/l-auteur                       premier jet, en attente de sa relecture
/fr/confidentialite                vérifiée dans le code de l'app, pas recopiée
/fr/conditions
/fr/lire                           le sommaire des 70 livres
/fr/lire/{livre}                   ses unités, avec le renvoi classique
/fr/lire/{livre}/{unité}?v=1-3     un passage — la route des liens partagés
/fr/lexique                        les 105 intraduisibles
/fr/lexique/{lemme}                la fiche — définition **et** occurrences
/.well-known/apple-app-site-association   application/json, aucune redirection
/sitemap.xml                       157 adresses, calculées, jamais écrites
```

### L'accueil montre, il n'annonce pas

La première version était un sommaire : trois liens vers trois pages maigres.
Un sommaire demande au lecteur de choisir avant de savoir de quoi il s'agit —
et sur un projet dont personne n'a entendu parler, il repart.

L'ordre de la page est un argument, et il ne se réorganise pas au hasard :

1. **L'affirmation** — la montagne en grand, une phrase, deux chemins.
2. **La démonstration** — Bereshit 1:1 chez Louis Segond (1910, domaine
   public), puis la restitution avec ses trois niveaux. C'est la pièce qui
   porte tout le site : l'écart se voit avant d'être compris. Citer une
   traduction moderne serait une contrefaçon ; une traduction ancienne et
   respectée rend d'ailleurs l'écart plus parlant.
3. **La clé de lecture** — les trois niveaux, expliqués après avoir été vus.
4. **Le verset du jour**.
5. **L'état du chantier** — 3 livres sur 70, et les chiffres viennent du
   `manifest.json` du pipeline, figés par `build.rs`. Jamais recopiés : un site
   qui annonce trois livres quand le vault en a cinq ment sans que personne ne
   le remarque.
6. **Qui traduit**, en trois lignes, puis le lien vers la page.
7. **Pour aller plus loin.**

`Hero` ouvre, `Bloc` fait tout le reste — une seule primitive de mise en page,
pleine largeur avec la mesure rétablie à l'intérieur. Deux primitives finissent
toujours par diverger sur un espacement.

**Reste à faire** : le déploiement, puis la bascule des domaines dans l'ordre
du §4. Le site est complet côté fonctionnalités ; ce qui reste est au §9.

## 8 bis. La liseuse

**Décidé le 13 août 2026** : une vraie liseuse en ligne, et les brouillons
**montrés avec la mention**. Six unités sur trente-neuf sont en brouillon ; les
cacher donnerait un corpus plus petit qu'il n'est, et un lien partagé vers un
chapitre en cours tomberait sur un 404 — ce qui se lit comme « ce texte
n'existe pas » alors qu'il existe.

### Le corpus est embarqué, pas lu

`build.rs` liste `dist/books/` et écrit un `include_str!` par livre. Deux
raisons de ne pas lire le disque à l'exécution : `dist/` ne doit jamais être
dupliqué, et un binaire sans dossier de données à côté ne peut pas tomber
parce qu'un déploiement l'aurait oublié.

Chaque livre a son `OnceLock` : **analysé à la première visite**, pas au
démarrage. Trois livres pèsent 912 Ko ; soixante-dix en pèseront vingt méga, et
les analyser au démarrage se paierait sur le démarrage à froid de la Lambda.
Le plan (`corpus.json`, 16 Ko) et le lexique (74 Ko) sont analysés au
démarrage — toute page en a besoin. Les occurrences (472 Ko) attendent la
première fiche.

### Les types que le pipeline produit

Relevés, pas devinés — le premier relevé n'avait parcouru que les chapitres et
manquait quatre types qui ne vivent que dans les définitions du lexique :

| blocs | nœuds |
|---|---|
| `verses` `heading` `list` `para` `quote` `table` `rule` | `text` `term` `important` `gloss` `em` `translit` `heb` `link` `break` |

`important` porte des **enfants**, pas une chaîne : un `==…==` peut contenir un
intraduisible, et l'aplatir perdrait le lien vers sa fiche en silence.

Les DTO tolèrent un type inconnu et l'omettent, pour qu'un ajout au pipeline ne
casse pas une page entière. Ce qui rend cette tolérance acceptable est le test
`tout_le_corpus_s_analyse_sans_type_inconnu` : il parcourt tout le corpus et
échoue si un type y échappe. La bascule se voit à `cargo test`, jamais en
production.

**Trois pièges de forme**, tous découverts par des tests qui échouaient :
`reference` est nul sur une introduction (elle ne recouvre aucun verset) ;
`verse` est nul sur 319 des 2033 occurrences (celles qui vivent dans un titre
ou une note) ; `hebrew`, `rendering`, `forms` sont nuls sur plusieurs fiches.
Un `#[serde(default)]` ne suffit pas — il couvre la clé absente, pas la clé
nulle.

### La sérialisation traverse le domaine, et c'est un arbitrage

`api.rs` pose que ce qui voyage est un **transport**, pas un type du domaine.
La règle tient pour le verset du jour, qui est plat. Elle est **écartée** pour
l'arbre du corpus : le recopier ferait deux énumérations récursives à tenir
d'accord et **trois** endroits à modifier au prochain type de nœud. Un `Noeud`
n'a aucun invariant que la sérialisation pourrait violer.

### La composition française est faite au rendu

Le corpus porte **2124 espaces ordinaires** devant `;` `:` `!` `?` `»` et pas
une seule insécable — le vault écrit des mots, pas de la composition. Une
espace ordinaire est un point de coupure : le navigateur y renvoie la
ponctuation à la ligne suivante. C'est arrivé sur la première capture de la
liseuse.

`design/verset.rs::composer` pose une **fine insécable** (U+202F) devant
`; ! ? »` et après `«`, une **insécable pleine** (U+00A0) devant `:` — l'usage
de l'Imprimerie nationale. Au rendu et non dans le pipeline : le vault est la
source du texte, la composition est une affaire d'affichage, et la corriger ici
la corrige pour les livres qui n'existent pas encore.

Défaut voisin, corrigé dans `Verset::corps()` : une glose se pose **avant** la
ponctuation qui suit, donc la retirer laissait « habitant , et la face » — 561
cas. On referme devant `, . ) ] …`, jamais devant `: ; ! ? »`, qui en veulent
une.

### Deux leçons de forme, déjà vues ailleurs

- Le fil d'Ariane **sépare** : sa barre oblique n'a rien à faire après le
  dernier maillon (`last:hidden`). « Lire / Bereshit / » se lisait comme un fil
  coupé.
- Un `dir="rtl"` sur un **bloc** en aligne aussi le contenu à droite : le nom
  hébreu partait se coller au bord de l'écran, détaché du titre qu'il double.
  Un `span` en ligne garde l'ordre des caractères sans l'alignement.

### Les réglages de lecture

Portés de l'app, `ReadingSettingsSheet` — mêmes bascules, mêmes libellés, mêmes
notes. Un lecteur qui passe du téléphone au site doit retrouver les mêmes mots.

| section | bascule | défaut |
|---|---|---|
| Disposition | Versets à la suite | non |
| Niveaux du texte | Gloses | oui |
| Niveaux du texte | Translittération et hébreu | oui |

**Il n'y a pas de bascule pour les intraduisibles**, et l'app n'en a pas non
plus. Son moteur de rendu dit pourquoi : « un terme important survit à
l'extinction des niveaux : il appartient au corps, pas à l'appareil critique ».
Un intraduisible n'est pas un commentaire ajouté au texte — c'est le texte,
qu'on a refusé de traduire. L'éteindre laisserait un trou, et la promesse de
l'or cesserait d'être tenue selon un réglage.

Ce que le site **n'emprunte pas** : taille du corps, interligne, fonte, thème.
L'app a raison de les offrir — elle est un lecteur, et un lecteur s'adapte à qui
le tient. Le site est une **édition** : sa nuit d'aubergine, son corps à 21 px
et sa Literata sont des décisions, pas des défauts qu'on propose de corriger.

**On retire les nœuds, on ne les masque pas.** Un `display: none` aurait laissé
« habitant , et la face » et des mots collés — le défaut déjà corrigé sur les
aperçus. `domaine/lecture.rs` retire, **fond** les fragments devenus voisins,
puis resserre les blancs. Les trois passes, dans cet ordre : sans la fusion, le
resserrage ne verrait rien, chaque espace étant seul dans son fragment et
parfaitement légitime.

Deux bugs trouvés par les tests en l'écrivant, et le second était grave :

- la fusion doit être **récursive** — un niveau 3 retiré dans une glose laisse
  deux fragments voisins *dans* la glose ;
- un fragment qui ne contient **qu'une espace** était vidé. C'est le fragment le
  plus courant du corpus, celui qui sépare un intraduisible de sa
  translittération : il collait les deux mots, et il le faisait même quand on
  n'éteignait rien.

`Verset::corps()` passe désormais par la même fonction : la règle
typographique n'existe **qu'une fois**.

**Le bouton flotte, la feuille monte.** Une première version posait le panneau
en haut du chapitre. Ça ne tenait pas : un chapitre fait jusqu'à quarante-six
versets, et l'on décide d'éteindre les gloses **au milieu** de la lecture. Un
réglage qu'il faut remonter chercher n'en est plus un. Le bouton « aA » reste
donc en bas, la feuille monte par-dessus le texte — comme celle de l'app.

Le retrait du bas ajoute `env(safe-area-inset-bottom)` : sans lui, le bouton se
pose sur la barre d'accueil d'un iPhone, où le geste de retour prend le clic en
premier.

**La feuille reste montée, elle n'est pas démontée.** C'est ce qui permet
d'animer la **fermeture** autant que l'ouverture : un `<Show>` arrache
l'élément du document, et rien ne peut plus transiter sur ce qui n'existe plus.
Une feuille qui monte doucement et disparaît d'un coup se remarque davantage
qu'une feuille jamais animée.

Le prix est qu'il faut la rendre inerte quand elle est fermée — `inert`, rendu
**absent** et non « faux » : c'est un attribut booléen, donc `inert="false"`
rendrait la feuille inerte tout autant. Sans lui, ses interrupteurs resteraient
dans l'ordre de tabulation et dans l'arbre d'accessibilité, invisibles mais
atteignables.

Le bouton, lui, `se-poser` — il monte de quelques pixels en arrivant. Il ne peut
pas être dans le HTML du serveur, donc il apparaît une seconde après la page, et
sans ce mouvement ça se lit comme un défaut de chargement. Toutes les
transitions portent `motion-reduce:transition-none` : le mouvement déclenche des
vertiges chez une partie des lecteurs, et c'est un réglage système, pas un
goût.

**Le panneau n'est rendu que par le navigateur.** Sans JavaScript, des
interrupteurs qui ne commutent rien seraient un mensonge — pire qu'une absence,
parce qu'on les essaie. Le serveur rend toujours tout : c'est le rendu honnête
pour qui n'a pas de JavaScript, et c'est ce qu'un moteur doit indexer. Les deux
côtés partent des défauts, seul le second bouge — pas de désaccord
d'hydratation.

Retenus dans `localStorage` sous `ont.lecture`, en JSON, avec `#[serde(default)]`
par champ : le jour où un quatrième réglage apparaît, les réglages déjà retenus
n'en portent pas la clé, et sans cette tolérance ils seraient tous jetés d'un
coup. L'app fait la même chose dans son `init(from:)`.

`web-sys` entre en dépendance directe pour la fonctionnalité `Storage` : Leptos
réexporte `web_sys` mais sans elle, chaque interface du navigateur étant derrière
son propre drapeau.

### La bannière d'app

`tete.rs::IDENTIFIANT_APP_STORE` vaut `None`, et tant qu'il vaut `None` la
balise n'est pas écrite. Le jour où la fiche existe dans App Store Connect,
coller son `Apple ID` — dix chiffres, attribué **à la création de la fiche**,
avant toute publication — allume la bannière sur toutes les pages.

Elle ne remplace pas les liens universels : ceux-là ouvrent l'app *directement*
sur `/fr/lire/*`. La bannière s'adresse à qui n'a **pas** l'app. Et elle
n'existe que dans Safari sur iOS ; le bandeau « Ouvrir dans la web app » de
macOS relève d'une autre mécanique, qu'aucune balise ne déclenche.

## 8 ter. Le cache, et pourquoi les fichiers portent une empreinte

`hash-files = true`. Les artefacts sortent nommés
`ontbible.<empreinte>.{js,wasm,css}`.

Ce n'est pas une optimisation, c'est une **correction**. Sans ça, `/pkg/` est
servi avec un simple `last-modified` et aucun `cache-control` : un navigateur
applique son cache heuristique et ne revalide pas. Un visiteur qui revient
exécute donc l'ancien WASM contre le nouveau HTML.

Et ça ne se voit pas. Le rendu du serveur reste juste, la page s'affiche, se
lit, s'indexe — seul manque **tout ce qui vient de l'hydratation**. C'est
arrivé le jour même sur le bouton de lecture : présent au simulateur, absent
dans le navigateur, et il a fallu vider le cache à la main. Derrière un CDN,
ça durerait des jours.

Deux pièges rencontrés en l'activant :

- `cargo-leptos` écrit le manifeste dans `target/<profil>/hash.txt`, mais le
  serveur le cherche depuis son **répertoire courant**. Il faut le lui donner ;
- les empreintes ne sont lues que si le binaire démarre avec
  `LEPTOS_HASH_FILES=true`. Lancé à la main sans elle, il retombe en silence
  sur les noms fixes — c'est-à-dire sur le défaut qu'on venait de corriger.
  **À poser dans l'environnement du déploiement.**

La feuille de style passe par `HashedStylesheet`, dans `shell` : le composant a
besoin des options, qui n'existent que de ce côté. Un `href` écrit en dur aurait
gardé son nom fixe pendant que le JS et le WASM prenaient le leur — le pire des
deux mondes.

## 8 quater. Le déploiement

**Le site est en ligne** — sur l'adresse de CloudFront, pas encore sur le
domaine :

```
https://d1158mwsz5tj2z.cloudfront.net
```

```
CloudFront ──┬── /pkg/*  /images/*  /fontes/*  /robots.txt ──▶  S3
             └── tout le reste ──▶  API Gateway ──▶  Lambda (arm64)
```

Un seul geste : `./scripts/deployer.sh`. Il construit, pousse, applique,
invalide.

**Mesuré** : 75 ms par page à chaud, 378 ms à froid. WASM 1 275 Ko en
`application/wasm`, première visite ~700 Ko sur le fil, visites suivantes 0.
Coût attendu : 0 € jusqu'à des dizaines de milliers de visites, ~2 €/mois si le
palier gratuit disparaissait. Une alerte de budget à 5 $ en prévision — c'est
la seule chose qui voie venir un abus, qui ne se manifeste par aucune erreur.

### Deux constructions, et ce n'est pas un choix

`cargo leptos build --release` **échoue sur macOS** :
`ld: Assertion failed: (name.size() <= maxLength)`. C'est une limite du linker
d'Apple sur la longueur des noms de symboles, que les génériques imbriqués de
Leptos font exploser.

Le script construit donc le front avec `cargo leptos --frontend-only` — qui pose
les empreintes et écrit `target/release/hash.txt` — et le serveur avec
`cargo lambda`, qui croise-compile pour Linux ARM **avec zig**. Le défaut ne
touche que le linker d'Apple ; le binaire déployé, lui, sort propre.

Le paquet Lambda porte le binaire **et** `hash.txt`. Sans ce second fichier, la
Lambda écrit `ontbible.js` dans le HTML — une adresse que le seau ne sert pas,
donc une page sans son WASM, et rien ne le signale.

### Trois pièges rencontrés, tous muets

**Les adresses de fonction Lambda sont bloquées sur ce compte.** Le montage
prévu — `AuthType = AWS_IAM` et un contrôle d'accès d'origine CloudFront — a
été écrit, appliqué, et rend « Forbidden ». Vérifié : permission conforme,
contrôle d'accès de type `lambda` en signature `always`, attaché à la bonne
origine. Diagnostic décisif : **l'appel direct échouait aussi**, avec
`AuthType = NONE` et une permission `Principal: "*"`, sur une adresse recréée
de zéro. Le compte les refuse, très probablement par une politique
d'organisation que `ont-app` n'a pas le droit de lire.

On passe donc par **API Gateway**, comme le backend de l'app sur ce même
compte. 1 $ par million de requêtes — douze centimes par mois à trente mille
visites.

**API Gateway refuse un `Host` qui n'est pas le sien.** Une politique de
requête d'origine maison transmettait le `Host` du visiteur : 403, sans un mot
d'explication. La politique gérée `AllViewerExceptHostHeader` est faite
exactement pour ça, et son nom le dit.

**Terraform supprime avant de mettre à jour.** Retirer un contrôle d'accès
encore référencé par la distribution échoue en boucle : il faut appliquer
d'abord `-target=aws_cloudfront_distribution.site`, puis le reste.

### Déployer casse le site local, et c'est normal

`target/site/pkg` est **le même dossier** des deux côtés. Un déploiement y pose
les fichiers de production ; le serveur local, lui, réclame ceux de
développement — dont l'empreinte diffère. La page arrive alors nue, sans style
ni WASM, et l'on croit avoir cassé quelque chose.

Il suffit de relancer `cargo leptos watch`. Le script le rappelle en dernière
ligne.

### Les caches ne se règlent pas au même endroit

| chemin | politique | pourquoi |
|---|---|---|
| `/pkg/*` | un an, `immutable` | le nom porte l'empreinte du contenu |
| `/images/*` `/fontes/*` | un jour | le nom est **fixe** : `logomark.svg` reste `logomark.svg` |
| le HTML | pas de cache | l'accueil porte le verset du jour, qui change à minuit |

`?v=` entre dans la clé de cache du HTML : deux liens partagés différents ne
doivent pas recevoir la même page, leurs métadonnées d'aperçu diffèrent.

Et `aws s3 sync` ne connaît pas `.wasm` : il le pose en
`binary/octet-stream`, et le navigateur refuse alors de le compiler en flux.
Le script repasse dessus avec `--content-type application/wasm`.

### L'identité AWS

`ont-app`, profil local `[ont]`, politique `ont-deploy` — celle qui servait
déjà à l'API, **étendue** plutôt que doublée : S3 borné à `ont-site-*`,
CloudFront, ACM en us-east-1 (CloudFront n'accepte ses certificats que de là),
budgets.

Le simulateur IAM a trouvé ce que l'œil laissait passer :
`acm:RequestCertificate` **ne peut pas** être borné à un ARN de certificat,
puisque l'ARN n'existe pas encore quand on le demande. Il est borné par région.

### Ce que le déploiement n'a **pas** fait

Il n'a pas touché au DNS. `ontbible.com` pointe toujours la Lambda de l'API.
La bascule suit l'ordre du §4, et **cet ordre n'est pas négociable** : créer
`api.ontbible.com`, y basculer `ONTAPIBaseURL` dans l'app, publier l'app,
vérifier, et seulement ensuite donner la racine au site. Inversé, les liens
universels déjà partagés tombent — et ça ne se voit pas tout de suite, iOS
gardant le fichier d'association en cache.

## 8 quinquies. Le déploiement continu

`.github/workflows/deployer.yml` — au `push` sur `main`, ou à la main.

**Aucune clé nulle part.** L'authentification passe par OIDC : GitHub signe un
jeton qui dit d'où vient le job, AWS le vérifie et prête `ont-site-github` pour
sa durée. La condition sur `sub` est la serrure — elle épingle
`repo:ONTBible/ONTBibleWebapp:ref:refs/heads/main`. Sans elle, n'importe quel
dépôt du monde pourrait emprunter ce rôle : le fournisseur ne dit que « ce jeton
vient bien de GitHub Actions », pas de qui.

Le fournisseur OIDC existait déjà sur le compte (un autre projet l'a créé) : on
le **référence**, on ne le recrée pas — AWS n'en accepte qu'un par émetteur.

**La CI ne touche jamais à Terraform.** L'état vit en local ; un job qui
l'exécuterait travaillerait sans savoir ce qui existe, donc recréerait tout ou
détruirait ce qu'il ignore. Le rôle n'a d'ailleurs pas les droits : il peut
poser des fichiers, remplacer le **code** de la Lambda, invalider — pas changer
sa configuration ni redessiner la distribution.

### Elle régénère le corpus, et ça répond à une vieille question

`dist/` est **gitignoré** dans ONTBibleApp : c'est un produit, pas une source.
Le cloner ne donnerait rien. La CI clone donc les **trois** dépôts côte à côte —
site, pipeline, vault — et rejoue `npm run build`.

Conséquence : le site déployé porte le vault **au moment où le job tourne**.
Corriger un verset, pousser, relancer le workflow — et la correction est en
ligne. C'est la propagation automatique qui n'existait pas (voir §8 bis) ;
elle s'arrête à la frontière de l'app iOS, qui garde sa copie compilée.

### Deux pièges que la CI doit éviter

- **`cargo lambda` même sur Linux.** Un binaire construit avec le `cargo build`
  d'Ubuntu se lie à sa glibc 2.39 ; Amazon Linux 2023 en a 2.34. La fonction
  refuse alors de démarrer, sans autre message qu'un `Runtime.InvalidEntrypoint`.
  Zig croise-compile vers la bonne.
- **`hash.txt` dans le paquet.** Sans lui la Lambda écrit `ontbible.js` au lieu
  de `ontbible.<empreinte>.js` — une adresse que le seau ne sert pas.

La dernière étape **vérifie que le site répond**. Un déploiement n'est pas fini
quand la commande rend la main, il est fini quand une page arrive : sans ça, un
binaire qui ne démarre pas passerait pour un succès.

### Le badge App Store ne se redessine pas

`app-store-fr.svg` vient de `tools.applemediaservices.com`. Les directives
marketing d'Apple l'exigent **non modifié** — ni recoloré, ni retitré, ni
recomposé — avec une hauteur minimale et une zone de respect. La variante
blanche est celle qu'elles prescrivent sur un fond sombre.

C'est pourquoi il n'est pas rendu par `Bouton` comme le reste du site : ce n'est
pas un bouton, c'est une marque qu'on nous prête.

Et le QR n'est pas un ornement. La page se lit surtout sur un grand écran ; l'app
s'installe sur un téléphone. Un badge cliqué depuis un ordinateur ouvre une page
web, il ne pose rien sur l'appareil qui compte. Le QR fait le pont, et disparaît
en dessous de `sm` — on ne scanne pas l'écran qu'on tient.

**`application.rs::PUBLIEE` vaut `false`** tant qu'Apple n'a pas approuvé la
1.0. La page affiche alors « en relecture » à la place du badge : un badge qui
mène à une page d'erreur est pire qu'une absence, on l'essaie et le projet a
l'air cassé. Un seul booléen à basculer le jour de l'approbation.

## 9. Ce qui reste à trancher

- Le **texte de la page auteur** — le jet est écrit, il attend sa relecture.
  **Rien de cette page ne doit être mis en ligne avant.**
- **Jost ou EB Garamond** pour le corps du site. Jost est en place, EB Garamond
  est chargée à côté : un seul jeton à changer pour comparer en direct.
- L'**adresse de contact**. `contact@ontbible.com` est écrite dans les pages
  légales ; elle n'existe pas encore. Cloudflare Email Routing la fait suivre
  gratuitement, en deux minutes.
- Une **image d'aperçu** dessinée pour 1200 × 630. Sans elle, une messagerie
  n'affiche qu'une vignette.
- Le **®** du combination mark. Il est sur le wordmark, donc sur **toutes les
  pages**. En France, apposer ® sur une marque non déposée à l'INPI relève de
  l'article L.716-9 du code de la propriété intellectuelle — ce n'est pas un
  risque commercial, c'est une infraction. Trois sorties : la marque est
  déposée ; elle va l'être (~190 € pour une classe) ; ou on produit une
  variante sans le sigle, à côté des fichiers d'origine.

## 10. Les dépôts voisins

| | |
|---|---|
| `ONTBible/ONTBibleTranslation` | le vault — la traduction. **Public.** 93 commits signés |
| `ONTBible/ONTBibleApp` | pipeline, app iOS, backend. **Public** |
| `ONTBible/ONTBibleWebapp` | ce dépôt |

Les trois portent le même ruleset : `main` protégée, passage par PR
obligatoire, **signatures exigées**, suppression de branche après fusion.
Signer les commits — `commit.gpgsign` et une clé SSH sont déjà configurés
globalement sur cette machine.
