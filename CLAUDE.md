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

État actuel — `ontbible.com` pointe vers la Lambda de l'**API** :

```
ontbible.com  →  CNAME  d-xdeopzr27e.execute-api.eu-west-3.amazonaws.com
                 (nuage GRIS chez Cloudflare — le proxy orange casse le TLS
                  du domaine personnalisé d'API Gateway)
```

**Cible** — le site prend la racine, l'API passe sur un sous-domaine :

| | |
|---|---|
| `ontbible.com` | le site (cette application) |
| `api.ontbible.com` | l'API de l'app (Lambda existante) |
| `labibleont.com` | redirige en 301 vers `ontbible.com`, chemin et paramètres préservés |

Deux routes doivent **impérativement** rester servies à la racine, parce
qu'elles sont aujourd'hui rendues par la Lambda de l'API et qu'elles cassent
si elles disparaissent :

- `/.well-known/apple-app-site-association` — **`application/json`, aucune
  redirection**. C'est ce fichier qui autorise l'app iOS à ouvrir les liens
  `ontbible.com`. Contenu exact à reprendre depuis
  `ONTBibleApp/backend/src/interface/web.rs` : `appIDs` vaut
  `N49VNC2G57.com.labibleont.ONT`, `components` vaut `/fr/lire/*`.
  iOS ne le relit qu'à l'installation de l'app et le met en cache via le CDN
  d'Apple — une erreur ici ne se voit pas tout de suite.
- `/fr/lire/{livre}/{unité}` — la page d'un passage, avec ses balises Open
  Graph. Le segment de langue est délibéré : il épargne une migration le jour
  d'une édition anglaise.

Migration à faire **dans cet ordre**, sinon les liens universels tombent :
créer `api.ontbible.com`, y basculer `ONTAPIBaseURL` dans l'app, vérifier,
puis seulement rediriger la racine vers le site.

## 5. La direction artistique

**« Un peu ancienne »** — sa demande. Le livre imprimé classique, pas le
pastiche : filets fins, capitales espacées, larges marges, papier plutôt
qu'écran.

### Couleurs — reprises du logo, pas inventées

| rôle | clair | sombre |
|---|---|---|
| aubergine (le fond des cartes, la marque) | `#421B26` | `#421B26` |
| aubergine profonde (le bas des dégradés) | `#2A1018` | `#2A1018` |
| or | `#CDBE83` | `#CDBE83` |
| or profond (sur fond clair) | `#A6874F` | `#CDBE83` |
| parchemin (le fond) | `#FAF5EB` | `#171417` |
| surface (carte posée sur le fond) | `#FFFCF6` | `#252226` |
| encre | `#29211C` | `#E0DBD4` |
| terme important | `#862742` | `#D87994` |
| titre de section | `#421B26` | `#CDBE83` |

Les opacités viennent de l'app, pas du jugé : le filet vaut l'encre à **10 %**
(16 % sur fond sombre), le niveau 2 à **62 %**, la glose compose à **0,86 ×** le
corps et l'hébreu à **1,08 ×**. Ce sont les valeurs d'`ONTColors` et
d'`ONTTypography` — les reprendre à l'œil donnerait un site qui *ressemble* à
l'app sans être elle.

**« Un peu ancienne, un peu mystique »** — sa précision du 12 août. D'où trois
utilitaires dans `style/main.css` :

- `voile-aubergine` — une lueur dorée haute, l'aubergine qui s'assombrit vers
  le bas. La lumière semble venir d'au-dessus du texte, comme dans une nef.
  Trois couches, pas une de plus : au-delà, un dégradé cesse d'être une
  atmosphère et devient un effet.
- `grain-page` — deux halos à 4 % sur le parchemin, fixes au défilement. Sous
  les 4 %, l'œil ne l'identifie pas comme une couleur, il le lit comme une
  matière.
- `filigrane-montagne` — la montagne très grande et très pâle derrière un
  bandeau. Elle n'est pas là pour être vue, elle est là pour qu'on sente qu'il
  y a quelque chose.

L'aubergine `#421B26` est relevée **au pixel** sur le combination mark. Il
l'appelle « le violet » — c'est bien cette couleur-là.

`#862742` mérite une explication : c'est l'aubergine **éclaircie à teinte
constante** (343°). L'aubergine d'origine a un écart perceptuel de ΔE 18 avec
l'encre — l'œil ne la distingue pas dans une ligne de texte. `#862742` est à
ΔE 39, l'or à 44 : les deux marquages se détachent avec la même force.

### Typographie — toutes OFL, déjà dans `ONTBibleApp/app/Resources/Fonts/`

**Deux voix, et elles ne se mélangent pas** — sa décision du 12 août.

| rôle | fonte |
|---|---|
| **le site** — titres et corps | **Jost** — la géométrique de l'édition imprimée et du combination mark |
| **une citation de l'ONT** — corps | **Literata**, exactement comme l'app |
| **une citation de l'ONT** — hébreu | **Ezra SIL**, la seule qui positionne niqqud et te'amim |

Un verset doit se lire ici comme sur le téléphone, sinon le lecteur voit deux
traductions là où il n'y en a qu'une.

**EB Garamond reste embarquée en comparaison**, le temps qu'il tranche pour le
corps du site : un seul jeton à changer dans `style/main.css`. Le jour de la
décision, la perdante sort de `scripts/fontes.sh`, du CSS et du dépôt — une
fonte que plus aucune règle ne nomme est un fichier porté pour rien.

**Licences** : Ezra SIL, Frank Ruhl Libre, EB Garamond, Literata, Spectral,
Source Serif 4, Newsreader et Jost sont OFL — redistribuables. **SBL Hebrew
relève d'un EULA propriétaire** et **Taamey Frank CLM d'une GPL dont
l'exception ne couvre que les documents composés, pas un binaire** : ces deux
là ne doivent jamais entrer dans un livrable web.

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
| `portrait-640.png`, `portrait-1024.png` | Gloire Bikouta, détouré sur transparence |

Les SVG sortent d'Affinity et passent par `scripts/normaliser-svg.py`, qui est
idempotent : à relancer après chaque nouvel export, sans réfléchir. Il pose la
`viewBox` que l'export omet — sans elle un vecteur ne se met pas à l'échelle,
ce qui lui retire sa raison d'être — retire la taille en pixels pour que la CSS
décide, et remplace la couleur en dur par `currentColor`, plus l'or de la
marque en attribut de présentation : une valeur par défaut qu'une règle CSS
écrase, donc la montagne suit le thème sombre tout en restant dorée quand le
fichier est ouvert seul.

L'or exporté par Affinity vaut `#CFBD7A`, pas `#CDBE83` — un décalage de profil
colorimétrique. C'est le second qui fait foi : il est relevé au pixel sur le
rendu de la marque.

Le portrait est un détourage propre sur alpha ; sur l'aubergine, la chemise
blanche tranche fort et le bas se dissout. Original en 4910×6844 dans
`Doneground/CV/done/photo CV/export/pro.png` si besoin d'un autre cadrage.

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

## 8. Où en est le site

**Fait** — squelette Leptos SSR, couches, design system, six pages, verset du
jour accordé à l'app, vecteurs de la marque normalisés, métadonnées complètes
(canonique, hreflang, Open Graph, JSON-LD).

```
/            → 307 vers /fr        (temporaire : « / » choisira la langue un jour)
/fr                                accueil — le principe, le verset du jour, les portes
/fr/le-pourquoi                    l'ontologie fonctionnelle, les trois niveaux montrés
/fr/ce-que-l-ont-n-est-pas         les cinq lignes du §10 du vault
/fr/l-auteur                       premier jet, en attente de sa relecture
/fr/confidentialite                vérifiée dans le code de l'app, pas recopiée
/fr/conditions
```

**Reste à faire**, dans cet ordre :

1. Les deux routes techniques — l'association et `/fr/lire/{livre}/{unité}` —
   portées depuis `ONTBibleApp/backend/src/interface/web.rs`.
2. La liseuse, si elle est décidée (§9) : elle demande un adaptateur qui lit
   `dist/` — publié comme artefact, jamais dupliqué.
3. Le lexique — `/fr/lexique/{lemme}`, que les intraduisibles pointent déjà.
4. Déploiement, puis bascule des domaines dans l'ordre du §4.

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
- Le site affiche-t-il **le corpus** (vraie liseuse en ligne) ou seulement les
  passages partagés ?
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
