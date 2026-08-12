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

**Leptos en mode SSR + Axum.** Tout en Rust, rendu côté serveur — sa demande
explicite.

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
| or | `#CDBE83` | `#CDBE83` |
| or profond (sur fond clair) | `#A6874F` | — |
| parchemin (le fond) | `#FAF5EB` | `#171417` |
| encre | `#29211C` | `#E0DBD4` |
| terme important | `#862742` | `#D87994` |

L'aubergine `#421B26` est relevée **au pixel** sur le combination mark. Il
l'appelle « le violet » — c'est bien cette couleur-là.

`#862742` mérite une explication : c'est l'aubergine **éclaircie à teinte
constante** (343°). L'aubergine d'origine a un écart perceptuel de ΔE 18 avec
l'encre — l'œil ne la distingue pas dans une ligne de texte. `#862742` est à
ΔE 39, l'or à 44 : les deux marquages se détachent avec la même force.

### Typographie — toutes OFL, déjà dans `ONTBibleApp/app/Resources/Fonts/`

| rôle | fonte |
|---|---|
| titres | **Frank Ruhl Libre Medium** — déjà les titres de l'app |
| corps | **EB Garamond** — la lettre du livre imprimé classique, la plus proche du registre « ancien » |
| hébreu | **Ezra SIL** — la seule qui positionne correctement niqqud et te'amim |

L'app compose son corps en **Literata** (dessinée pour l'écran). Pour le site
je proposerais EB Garamond, plus juste pour la direction demandée — mais c'est
un choix à lui soumettre, il a passé du temps sur cette comparaison.

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
| `montagne-512.png`, `montagne-1024.png` | la marque — **favicon**, et signe de section |
| `combination-mark.png` | le logo complet, 3000×2998 |
| `portrait-640.png`, `portrait-1024.png` | Gloire Bikouta, détouré sur transparence |

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

**Le verset du jour** est une *fonction de la date*, pas un tirage : l'app, le
widget et la notification tombent sur le même verset le même jour sans se
parler. Si le site l'affiche, il doit employer **la même fonction** —
`DailySelection` dans `ONTBibleApp/app/Packages/ONTKit/.../DailyVerse.swift`,
à porter en Rust : un pas fixe premier avec la taille du vivier, ce qui
garantit qu'aucun verset ne revient avant que tous soient passés. Le vivier
est `dist/daily.json` (251 versets, unités verrouillées uniquement — §12 : un
brouillon ne fait pas référence).

## 8. Par où commencer

1. `cargo leptos new` en mode SSR, axum. Vérifier que ça tourne en local
   avant toute mise en forme.
2. Le design system d'abord — jetons de couleur, fontes, filets, échelle
   typographique. Une page de démonstration qui les montre tous.
3. La page **L'auteur**, en premier et seule : c'est celle dont le registre
   doit être validé. Les autres suivent vite une fois le ton juste.
4. Les deux routes techniques — association et `/fr/lire/…` — portées depuis
   `ONTBibleApp/backend/src/interface/web.rs`.
5. Déploiement, puis bascule des domaines dans l'ordre du §4.

## 9. Ce qui reste à trancher

- **EB Garamond ou Literata** pour le corps du site.
- Le **texte de la page auteur** — lui, après un premier jet.
- Le **SVG de la montagne**.
- Le site affiche-t-il **le corpus** (vraie liseuse en ligne) ou seulement les
  passages partagés ? Si liseuse, il faut `dist/` — publié comme artefact
  depuis `ONTBibleApp`, jamais dupliqué.
- Le **®** du combination mark : s'il n'est pas déposé à l'INPI, l'afficher
  est un risque juridique. À vérifier avant de mettre le logo en ligne.

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
