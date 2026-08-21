# ontbible.com — le site de La Bible ONT

Le site public de **La Bible ONT**, une restitution française du corpus hébreu
et araméen antique fondée sur l'ontologie hébraïque fonctionnelle.

Il fait quatre choses, dans cet ordre d'importance :

1. **Dire le pourquoi.** Quelqu'un qui découvre l'ONT doit pouvoir comprendre de
   quoi il s'agit — l'ontologie fonctionnelle, les trois niveaux du texte, ce
   que l'ONT n'est pas.
2. **Donner le corpus à lire.** Une vraie liseuse : les 70 slots, les unités, le
   lexique des intraduisibles, et la route des liens partagés depuis l'app.
3. **Servir ce que l'app réclame** — le fichier d'association des liens
   universels, et le corpus publié que l'app télécharge.
4. **Fournir ce que l'App Store exige** : confidentialité, conditions,
   assistance, une adresse.

**En ligne, sur la racine du domaine.** `www`, `labibleont.com` et son `www`
renvoient ici en 301, chemin et paramètres préservés.

```bash
cargo leptos watch          # http://127.0.0.1:3000
cargo test --features ssr   # 69 tests
./scripts/deployer.sh       # construit, pousse, applique, invalide
```

---

## La pile — tout en Rust

**Leptos 0.8 en SSR + Axum, Tailwind v4.** Rendu côté serveur. `cargo-leptos`
télécharge lui-même le binaire Tailwind : il n'y a ni `node_modules`, ni
`tailwind.config.js` — la v4 se configure en CSS, dans `style/main.css`.

**Les couches, et une flèche ne remonte jamais :**

```
domaine        ──▶  rien                  pur — compile aussi en wasm
application    ──▶  domaine               déclare des ports (traits)
infrastructure ──▶  application, domaine  les réalise — `ssr` seulement
interface      ──▶  application, domaine  design/ (la forme) + pages/ (le propos)
main.rs        ──▶  tout                  la racine de composition
```

Le domaine ne prend même pas de bibliothèque de dates : il reçoit un numéro de
jour. C'est ce qui le rend testable sans horloge, et identique des deux côtés du
réseau.

**Un composant par fichier.** `design/` porte la forme, `pages/` le propos. Une
page n'écrit aucune valeur de style — si une forme manque, elle se crée dans
`design/`. Depuis Tailwind, la forme vit **dans** le `.rs`, à côté du balisage :
il n'y a plus de feuille parallèle qui puisse diverger.

---

## Le corpus est embarqué à la compilation

`build.rs` liste `../ONTBibleApp/dist/books/` et écrit un `include_str!` par
livre. **Jamais copié** — sinon la règle éditoriale du pipeline et celle du site
divergeraient.

Deux raisons de ne pas lire le disque à l'exécution : `dist/` ne doit exister
qu'une fois, et un binaire sans dossier de données à côté ne peut pas tomber
parce qu'un déploiement l'aurait oublié.

Chaque livre a son `OnceLock` : **analysé à la première visite**, pas au
démarrage. Trois livres pèsent 912 Ko ; soixante-dix en pèseront vingt méga, et
les analyser au démarrage se paierait sur le démarrage à froid de la Lambda. Le
plan (16 Ko) et le lexique (74 Ko) sont analysés au démarrage — toute page en a
besoin. Les occurrences (472 Ko) attendent la première fiche.

Les DTO tolèrent un type de nœud inconnu et l'omettent, pour qu'un ajout au
pipeline ne casse pas une page entière. Ce qui rend cette tolérance acceptable
est le test `tout_le_corpus_s_analyse_sans_type_inconnu` : il parcourt tout le
corpus et échoue si un type y échappe. La bascule se voit à `cargo test`, jamais
en production.

### Le verset du jour tombe le même que sur le téléphone

C'est une **fonction de la date**, pas un tirage : l'app, le widget, la
notification et le site tombent sur le même verset sans se parler.

Deux pièges, tenus par des témoins relevés en **exécutant le Swift** :

- **La permutation** — un pas fixe premier avec la taille du vivier, valant
  ~0,618 × celle-ci. Aucun verset ne revient avant que tous soient passés, là où
  un tirage donnerait un doublon dans le mois avec quatre chances sur cinq.
- **Le décalage d'un jour** — l'app prend minuit **local** puis divise cet
  horodatage comme s'il était UTC. À l'est de Greenwich, le numéro de jour vaut
  la date civile moins un. `infrastructure/horloge.rs` **reproduit** ce calcul :
  le corriger ferait diverger le site du téléphone du même lecteur.

---

## Les routes

```
/            → 307 vers /fr        (« / » choisira la langue un jour)
/fr                                l'accueil — montre, n'annonce pas
/fr/le-pourquoi                    l'essai de fond
/fr/ce-que-l-ont-n-est-pas         les cinq lignes du §10 du vault
/fr/l-auteur
/fr/l-app                          installer l'app — badge, QR, TestFlight
/fr/lire                           le sommaire des 70 livres
/fr/lire/{livre}                   ses unités, avec le renvoi classique
/fr/lire/{livre}/{unité}?v=1-3     un passage — la route des liens partagés
/fr/lexique                        les intraduisibles
/fr/lexique/{lemme}                la fiche — définition **et** occurrences
/fr/confidentialite  /fr/conditions  /fr/assistance
/.well-known/apple-app-site-association   application/json, aucune redirection
/manifest.webmanifest              ce qui rend le site installable sur Android
/sitemap.xml                       calculé depuis le corpus, jamais écrit à la main
```

Le segment de langue est délibéré : il épargne une migration le jour d'une
édition anglaise.

**Les brouillons sont montrés, avec la mention.** Six unités sur trente-neuf
sont en cours ; les cacher donnerait un corpus plus petit qu'il n'est, et un
lien partagé vers un chapitre en cours tomberait sur un 404 — ce qui se lit
comme « ce texte n'existe pas » alors qu'il existe.

---

## La direction artistique

**Une nuit d'aubergine**, et cette peau porte un nom : **mystique**. L'app la
propose en quatrième thème de sa liseuse et transpose ses valeurs —
`ONTColors.nuit`, `nuitSurface`, `nuitEncre`. **C'est ici la référence** : une
teinte se retouche dans `style/main.css`, puis se reporte dans l'app. Jamais
l'inverse.

La rampe est **dérivée de la marque** : teinte constante 343°, celle de
`#421B26` relevée au pixel ; seule la clarté varie. Les valeurs sont calculées,
pas choisies à l'œil.

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
| `accentuation` | `#D87994` | l'accentuation — 6,5:1 |

**Jamais de blanc pur, jamais un gris.** La halation fait déborder un texte
clair dans le noir : les lettres paraissent plus grasses et se doublent sur une
dalle OLED. Descendre à 87 % supprime le halo en restant au-delà du seuil AAA,
et l'encre garde la chaleur du parchemin de l'app.

Trois conséquences dans la feuille, et ce ne sont pas des goûts : le lissage
forcé (sur fond sombre, il compense exactement le débordement optique), un
souffle d'interlettrage, et **`grain-page`** — un bruit `feTurbulence` à 3,5 %
qui casse les bandes d'un dégradé sombre. Un écran n'a que 256 valeurs par
canal, et l'écart entre deux nuances de nuit est plus petit que ça.

| rôle | fonte |
|---|---|
| titres, navigation, capitales | **Jost** — la géométrique du combination mark |
| tout le reste, citations comprises | **Literata** — celle du corps de l'app |
| hébreu | **Ezra SIL** — la seule qui positionne niqqud et te'amim |

Le corps est à **21 px**, et la mesure suit à 38 rem. EB Garamond a été essayée
puis retirée : ses déliés bavent sur fond sombre. Ne pas la reproposer sans lire
le CLAUDE.md §5.

**Deux règles valent pour tout ce qu'on ajoutera** : une taille en pixels est une
taille juste sur un seul écran — la marque, l'horizon, la perspective se
mesurent en part de fenêtre ; et une échelle se tient aux deux bouts, par
construction et non à l'œil.

---

## Les trois niveaux du texte

C'est la raison d'être de tout le pipeline (voir `../ONTBibleApp/README.md`).

| dans le `.md` | rôle | rendu |
|---|---|---|
| texte nu | niveau 1 — le corps | encre |
| `**mot**` | intraduisible | **or**, cliquable vers sa fiche |
| `==mot==` | accentuation | **`#862742`**, semi-gras, **inerte** |
| `*[glose]*` | niveau 2 | plus petit, encre atténuée |
| `(*translit* / hébreu)` | niveau 3 | italique + Ezra SIL, isolation bidi FSI/PDI |

L'or **promet** une fiche et la tient. Le bordeaux clair marque sans rien
promettre.

Les réglages de lecture sont portés de l'app — mêmes bascules, mêmes libellés.
Ce que le site **n'emprunte pas** : taille, interligne, fonte, thème. L'app est
un lecteur, et un lecteur s'adapte à qui le tient ; le site est une **édition**.

**On retire les nœuds, on ne les masque pas.** `domaine/lecture.rs` retire, fond
les fragments devenus voisins, puis resserre les blancs — dans cet ordre, sinon
le resserrage ne voit rien. Un `display: none` laisserait « habitant , et la
face ».

Et la composition française est faite **au rendu** : le corpus porte 2124 espaces
ordinaires devant `; : ! ? »` et pas une seule insécable. `design/verset.rs::composer`
pose une fine insécable devant `; ! ? »`, une pleine devant `:`. Toute chaîne du
corpus posée dans une page y passe — `./scripts/verifier-composition.py` le
vérifie sur le **rendu**, seul endroit qui voie les littéraux Rust coupés en
deux.

---

## Voir le site avant de le déployer

Trois outils, et ils ne servent pas à la même chose.

| | |
|---|---|
| `./scripts/dev-sync-empreintes.sh` | **d'abord, toujours** — voir ci-dessous |
| `scripts/apercu.py` | rend une page par QuickLook — juger une composition sur grand écran |
| `scripts/sim.sh /fr/l-auteur` | ouvre la page dans le **simulateur iOS** et la capture |
| `scripts/verifier-composition.py` | échoue sur une ponctuation double qui peut tomber à la ligne |

**`sim.sh` est la seule vérification qui vaille pour tout ce qui dépend de la
largeur.** QuickLook rend à une fenêtre fixe puis réduit l'image : les requêtes
média y voient toujours un grand écran. Une échelle typographique qui
s'inversait sous 1024 px a compilé, passé les tests, et n'a été vue qu'au
simulateur.

**Et `dev-sync-empreintes.sh` n'est pas une formalité.** En mode `watch`,
cargo-leptos régénère `target/site/pkg/ontbible.{css,js,wasm}` mais **pas** les
copies empreintées — et ce sont celles-là que le serveur écrit dans le HTML. La
page servie porte donc les assets du dernier redémarrage complet. On corrige un
jeton, on recharge, rien ne bouge, et l'on croit que la correction est fausse.
Une heure a été perdue là-dessus. Compilé en debug, le serveur déclare
`no-store` sur tout, ce qui règle le second étage du même piège — Safari
resservait sa copie sans revalider, le nom empreinté n'ayant pas changé.

---

## Le déploiement

```
CloudFront ──┬── /pkg/*  /images/*  /fontes/*  /corpus/*  /robots.txt ──▶  S3
             └── tout le reste ──▶  API Gateway ──▶  Lambda arm64 (eu-west-3)
```

Un seul geste : `./scripts/deployer.sh`. Il construit, pousse, applique,
invalide. **Mesuré** : 75 ms par page à chaud, 378 ms à froid.

**Deux constructions, et ce n'est pas un choix.** `cargo leptos build --release`
échoue sur macOS — le linker d'Apple bute sur la longueur des symboles que les
génériques imbriqués de Leptos produisent. Le script construit donc le front
avec `--frontend-only` et le serveur avec `cargo lambda`, qui croise-compile
pour Linux ARM avec zig. Même sur un runner Linux : un binaire lié à la glibc
2.39 d'Ubuntu ne démarre pas sur Amazon Linux 2023, qui en a 2.34.

Le paquet Lambda porte le binaire **et** `hash.txt`. Sans ce second fichier, la
Lambda écrit `ontbible.js` dans le HTML — une adresse que le seau ne sert pas,
donc une page sans son WASM, et rien ne le signale.

**Déployer casse le site local, et c'est normal** : `target/site/pkg` est le même
dossier des deux côtés. Relancer `cargo leptos watch`.

### Les caches ne se règlent pas au même endroit

| chemin | politique | pourquoi |
|---|---|---|
| `/pkg/*` `/corpus/*` | un an, `immutable` | le nom porte l'empreinte du contenu |
| `/images/*` `/fontes/*` | un jour | le nom est **fixe** : `logomark.svg` reste `logomark.svg` |
| le HTML | pas de cache | l'accueil porte le verset du jour, qui change à minuit |

Les images portent donc leur empreinte **en paramètre** (`?v=`), calculé par
`build.rs` sur le contenu. Deux choses à savoir : `cargo leptos watch` ne
repasse pas par `build.rs` quand une image change — il faut un `cargo leptos
build` pour voir le vrai comportement ; et l'invalidation CloudFront reste
nécessaire, la politique de cache du bord n'entrant pas les paramètres dans sa
clé.

### La CI

| workflow | quand | ce qu'il fait |
|---|---|---|
| `eprouver.yml` | proposition vers `main` | tout jusqu'à la construction, plus cinq routes interrogées pour de vrai |
| `deployer.yml` | `push` sur `main`, ou à la main | construit, pousse, invalide, **et vérifie que le site répond** |

**Aucune clé nulle part** : OIDC, et la condition sur `sub` épingle
`repo:ONTBible/ONTBibleWebapp:ref:refs/heads/main`. C'est la serrure — sans elle,
n'importe quel dépôt pourrait emprunter le rôle.

**La CI ne touche jamais à Terraform.** L'état vit en local ; un job qui
l'exécuterait travaillerait sans savoir ce qui existe. Le rôle n'a d'ailleurs pas
les droits : poser des fichiers, remplacer le **code** de la Lambda, invalider.

Et elle **régénère le corpus** : elle clone les trois dépôts côte à côte et
rejoue le pipeline. Le site déployé porte donc le vault au moment où le job
tourne — corriger un verset, pousser, et la correction est en ligne.

---

## Ce que le site sert à l'app

| adresse | pour quoi |
|---|---|
| `/.well-known/apple-app-site-association` | les liens universels — `application/json`, **aucune redirection** |
| `/fr/lire/{livre}/{unité}` | la page de repli d'un passage partagé, avec ses balises Open Graph |
| `/corpus/` | le corpus publié, que l'app télécharge — `scripts/corpus-publie.py` |

iOS ne relit l'association qu'à l'installation, et Apple la met en cache via son
propre CDN : une erreur ici ne se voit pas tout de suite.

Le corpus publié porte l'empreinte **dans le nom** de chaque fichier, ce qui
autorise un cache d'un an sans risque. Le manifeste est la seule exception : nom
fixe, cache court — c'est le point d'entrée.

---

## Les ressources

Dans `public/images/`, et pas dans un `assets/` séparé : `public/` est ce que
`cargo-leptos` recopie tel quel, donc une image y est à la fois la source et le
livrable. Deux dossiers obligeraient à les synchroniser, et l'un des deux
finirait périmé.

Les SVG sortent d'Affinity et passent par deux scripts **idempotents**, à
relancer après chaque export sans réfléchir : `normaliser-svg.py` (pose la
`viewBox` que l'export omet, retire la taille en pixels, remplace la couleur en
dur par `currentColor`) puis `retirer-le-r.py`, qui retire le ® tant que la
marque n'est pas déposée.

`images-sociales.py` compose l'aperçu et l'icône **depuis les jetons du site** —
une image dessinée à part trahit le jour où la marque bouge. `portrait.py`
détoure depuis le fichier brut, qui n'est pas dans le dépôt : c'est la seule
source, gardez-le. `qr-app.py` engendre le QR — et un QR ne se juge pas à l'œil,
il se décode.

Le badge App Store, lui, **ne se redessine pas** : les directives d'Apple
l'exigent non modifié. C'est pourquoi il n'est pas rendu par `Bouton` comme le
reste du site — ce n'est pas un bouton, c'est une marque qu'on nous prête.

---

## Ce qui reste ouvert

- **La page de l'auteur** attend sa relecture. Rien n'en va en ligne avant.
- **Le dépôt de la marque à l'INPI** — une marque verbale, « La Bible ONT »,
  classes 9, 16 et 41, 270 €. Le ® ne redevient légitime qu'à
  l'**enregistrement**, pas au dépôt : d'ici là il reste retiré des vecteurs, et
  les originaux attendent sous `*-avec-r.svg`.
- **La bannière Safari** (`tete.rs::IDENTIFIANT_APP_STORE`) et le badge
  (`application.rs::PUBLIEE`) s'allument le jour de l'approbation de la 1.0.
  C'est la même nouvelle dite à deux endroits.

---

## Le journal des décisions

**`CLAUDE.md`** — 119 Ko. Ce README dit ce que le site est ; le CLAUDE.md dit
*pourquoi il est ainsi*, et ce qu'il en a coûté de le découvrir : la bascule des
domaines et ses dix minutes de panne, les sept enregistrements du courrier et
leurs trois pièges, les quatre dessins de seuil comparés avant d'en retenir un,
et une bonne part des défauts qui compilent, passent les tests, et se voient
seulement à l'œil.

À lire avant de défaire quelque chose qui a l'air arbitraire.

## Les dépôts voisins

| | |
|---|---|
| [`ONTBibleTranslation`](https://github.com/ONTBible/ONTBibleTranslation) | le vault — la traduction |
| [`ONTBibleApp`](https://github.com/ONTBible/ONTBibleApp) | le pipeline, l'app iOS, le backend |

Les trois portent le même ruleset : `main` protégée, passage par pull request,
**signatures exigées**, suppression de la branche après fusion.
