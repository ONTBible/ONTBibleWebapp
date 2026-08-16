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

### Le courrier — Purelymail, depuis le 16 août 2026

`contact@ontbible.com` existe. Elle est écrite depuis longtemps dans les pages
légales et l'assistance ; elle ne recevait rien, et le §9 la portait comme une
décision en attente. Elle est faite.

**Une boîte dédiée, pas un alias.** C'est ce qui a écarté le domaine
personnalisé d'iCloud+, décidé le 14 août puis abandonné : il pose une adresse
**sur la boîte personnelle de l'auteur**. Le courrier du projet arrive dans le
courrier privé, et le jour où quelqu'un doit aider à répondre aux demandes
RGPD, il faudrait lui donner l'identifiant Apple. Purelymail donne une boîte
séparée, avec son mot de passe, lisible dans Mail.app par IMAP.

**19 $ chez Migadu ont été écartés pour une seule ligne du tarif** : vingt
envois par jour sur son plan d'entrée. Une journée d'ouverture de bêta où l'on
répond à vingt personnes suffit à le saturer. Purelymail coûte 10 $ par an,
sans plafond quotidien, avec adresses et domaines illimités — `rgpd@`,
`presse@` viendront sans rien repayer.

Zoho a un plan gratuit et il a été écarté aussi : **pas d'IMAP**, donc jamais
dans Mail.app. Proton l'a été pour la même raison sur iPhone, son Bridge étant
une application de bureau.

#### Les sept enregistrements, et les trois pièges

| Type | Nom | Valeur |
|---|---|---|
| TXT | `@` | `purelymail_ownership_proof=…` |
| MX | `@` | `mailserver.purelymail.com` — priorité 50 |
| TXT | `@` | `v=spf1 include:_spf.purelymail.com ~all` |
| CNAME | `purelymail1._domainkey` | `key1.dkimroot.purelymail.com` |
| CNAME | `purelymail2._domainkey` | `key2.dkimroot.purelymail.com` |
| CNAME | `purelymail3._domainkey` | `key3.dkimroot.purelymail.com` |
| CNAME | `_dmarc` | `dmarcroot.purelymail.com` |

**Le nuage gris, encore.** C'est le même piège qu'au reste du §4, et il coûte
ici le courrier entier : proxifié, Cloudflare répond à la place de Purelymail,
la signature DKIM ne se vérifie plus, et tout part en indésirable sans qu'aucune
erreur ne le dise. Les quatre CNAME sont en **DNS uniquement**.

**Un seul SPF, pour toujours.** Deux enregistrements SPF sur un domaine font un
SPF *invalide* — pas deux politiques, aucune. Le jour où un second expéditeur
arrive, on **fusionne les `include:`** dans le TXT existant ; on n'en ajoute
jamais un second.

**Le DMARC est en CNAME, donc c'est Purelymail qui écrit la politique.** Elle
vaut aujourd'hui `p=reject` — la plus stricte : tout courrier prétendant venir
de `@ontbible.com` sans passer par eux est **rejeté**, pas mis en quarantaine.
Excellent tant qu'ils sont le seul expéditeur, et c'est le cas. Mais le jour où
l'app enverra un mot de passe oublié, ou qu'une infolettre partira vers les
testeurs, **il faut ajouter cet expéditeur au SPF avant de l'allumer**, sinon
rien ne part et le rejet est silencieux du côté de l'expéditeur.

Un TXT `_dmarc` maison avait été proposé, pour garder la politique et recevoir
les rapports. Écarté : les rapports d'agrégat sont du XML illisible sans outil,
ils tomberaient dans la boîte de contact sans que personne ne les ouvre, et
Purelymail les lit à notre place en ne prévenant qu'en cas d'usurpation. Garder
la main sur ce qu'on ne regarde pas n'est pas un gain, c'est une charge.

#### Ce qui a failli passer inaperçu

L'enregistrement de propriété a d'abord été publié avec le **chemin d'un fichier
CleanShot** — le presse-papier contenait autre chose au moment du collage. Le
domaine ne validait pas, et le nom d'utilisateur du Mac se retrouvait lisible
par quiconque interrogeait le domaine. Trouvé en interrogeant le DNS depuis
l'extérieur, pas en relisant l'écran.

D'où la règle : **on ne juge pas un enregistrement DNS sur ce qu'on croit avoir
collé.** Après toute modification :

    dig +short MX ontbible.com
    dig +short TXT ontbible.com
    dig +short TXT purelymail1._domainkey.ontbible.com

La dernière est la seule qui prouve la chaîne entière — elle doit rendre une
clé `v=DKIM1; k=rsa; p=…`, pas seulement le CNAME.

#### Les réglages

IMAP `imap.purelymail.com:993`, SMTP `smtp.purelymail.com:465`, nom
d'utilisateur = l'adresse entière. Sur iPhone, Mail ne fait pas de Push pour un
compte IMAP générique : c'est de la récupération, quinze minutes au minimum. Sur
Mac, l'IMAP IDLE tient, donc c'est immédiat.

Et « Allow Account Reset » est **actif** côté Purelymail : quiconque contrôle le
DNS du domaine peut reprendre le compte. C'est assumé — le risque d'un mot de
passe perdu chez un hébergeur de cette taille est plus grand que celui d'une
attaque ciblée sur le Cloudflare — **à condition que ce Cloudflare garde sa
double authentification**. Si elle tombe, cette case doit tomber avec.

## 5. La direction artistique

**« Un peu ancienne »** — sa demande. Le livre imprimé classique, pas le
pastiche : filets fins, capitales espacées, larges marges, papier plutôt
qu'écran.

### Le site est une nuit d'aubergine

Cette peau porte un nom : **mystique**. Elle n'en avait pas tant qu'elle était
seule — on ne nomme pas ce qu'on ne distingue de rien. Elle en a un depuis que
l'app la propose en quatrième thème de sa liseuse, à côté de Parchemin, Clair
et Sombre : là, il fallait bien l'appeler quelque chose dans un menu.

Le nom est le même des deux côtés et doit le rester. Côté app, c'est
`ReadingTheme.mystique` dans `ONTKit/Reader/Reader.swift`, et la palette y est
transposée depuis ce dépôt-ci — `ONTColors.nuit`, `nuitSurface` et `nuitEncre`
citent les jetons ci-dessous par leur nom et leur valeur. **C'est ici la
référence** : une teinte se retouche dans `style/main.css`, puis se reporte
dans l'app, jamais l'inverse.

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
| `text-lg` → `text-3xl` | quarte juste sur grand écran, tierce sur téléphone |

**Une échelle se tient aux deux bouts, ou elle ne se tient pas. Corrigé le
14 août 2026.** Chaque palier portait son propre plancher, choisi à l'œil et
sans regarder le voisin : celui de `3xl` valait 2,25 rem, celui de `2xl`
2,35 rem — **l'échelle s'inversait**. Sur un téléphone, le titre de l'accueil
mesurait 41,9 px et la phrase qui le suit 39,7 px, soit un rapport de 1,055 là
où le grand écran en donne 1,333. Les deux lignes pesaient pareil et la page
n'avait plus de premier mot.

Ça ne pouvait pas se voir sur un grand écran : les planchers ne mordent que
sous 1024 px. Un défaut de ce genre vit dans la moitié de la page qu'on ne
regarde jamais.

Les paliers sont maintenant **une seule interpolation** de 390 à 1280 px.
Les plafonds n'ont pas bougé d'un dixième — le grand écran est exactement celui
d'avant. Les planchers suivent une tierce (1,2) : un rapport se resserre sur un
téléphone, il ne s'y renverse pas.

La propriété se vérifie sans essai, et c'est la règle à appliquer au prochain
palier : la valeur préférée de chaque taille domine celle d'en dessous terme à
terme, et ses deux bornes aussi. `clamp` étant monotone, l'ordre tient alors à
**toute** largeur. Jamais à l'œil.

### La marque aussi se mesure en part d'écran

Le wordmark était posé à 224 px en dur : 17 % de la largeur sur un ordinateur,
**56 %** sur un téléphone. Un rapport de un à trois, sur la pièce la plus
visible de la page. Le lecteur ne mesure rien, mais il voit une enseigne là où
il devrait voir une signature — et le titre, qui porte le propos, passait au
second rang sur le seul écran où il n'a pas la place de se défendre.

Le jeton `--container-marque` l'interpole comme l'échelle interpole le texte :
152 px à 390, 224 px à 1280. Le grand écran ne bouge pas. La règle vaut pour
toute image d'interface — une taille en pixels est une taille juste sur un seul
écran.

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


### Le seuil — « Entrer » ouvre une porte, le 16 août 2026

Sa femme l'a dit mieux qu'aucun test : « on parle d'un Temple juste avant, et
je suis déçue de ne pas voir une porte s'ouvrir ». Le bouton disait « Entrer »
depuis le premier jour et sautait à une ancre. Le mot promettait un seuil ; la
page n'en franchissait aucun.

`design/porte.rs` pose la scène, `style/main.css` la fait vivre sous « Le
seuil ». Deux vantaux s'écartent, la caméra pousse à travers, et le verset du
jour est derrière.

**Derrière, c'est le verset lui-même — pas un décor.** Le premier montage
posait la scène *avant* le bloc « Aujourd'hui » et lui donnait un décor à elle,
la voûte et l'horizon. On ouvrait donc sur une image, puis on arrivait sur le
verset : deux temps là où il n'en fallait qu'un, et la porte ne donnait sur
rien. `Porte` **enveloppe** maintenant le `Bloc`, qui reste à sa place dans le
document — dans le HTML du serveur, lisible par un moteur, lisible par un
lecteur d'écran, et simplement là quand la porte n'existe pas. C'est ce qui
rend l'ornement acceptable : il ne s'interpose devant rien qu'il n'ait d'abord
porté.

**Le clic et le défilement jouent la même chose.** La scène est pilotée par la
position de défilement ; le clic ne fait donc que **déplacer le défilement**, et
il n'y a pas deux animations à tenir d'accord.

Le saut d'ancre natif y suffisait presque — `scroll-behavior: smooth` est posé
sur `html`. Presque : sa durée est **fixe et courte**, trois à cinq dixièmes de
seconde quelle que soit la distance, et aucun réglage CSS ne la gouverne. La
porte claquait. `porte::traverser` ne fait rien d'autre que rendre sa durée au
mouvement — 1,6 s, en cubique adoucie aux deux bouts.

Et il **se désiste** quand il n'y a pas de seuil : si la scène ne dépasse pas
la fenêtre, il rend la main sans empêcher le comportement par défaut, et le
navigateur fait son saut. C'est une **mesure** et non une requête média, et
c'est ce qui la rend juste — elle constate l'état de la page au lieu de rejouer
le raisonnement de la feuille. Deux endroits qui décident la même chose
finissent toujours par en décider deux différentes.

**`Bouton` a gagné le tri de `Lien` au passage**, qu'il n'avait qu'à moitié :
une ancre reste une ancre, elle ne passe pas par le routeur. Ça marchait — le
routeur retombe sur `scrollIntoView` — mais par accident, et son gestionnaire
étant délégué au document, il passait après : impossible d'agir sur le clic.

**Zéro JavaScript, zéro bibliothèque 3D.** Deux couches composées par le GPU.
Le WASM ne grossit pas d'un octet, et le rendu du serveur ne change pas.

**Le repli est le défaut, et il est total.** Firefox stable garde les animations
au défilement derrière un drapeau — environ un lecteur sur six. Là, et chez qui
a demandé moins de mouvement, la scène n'existe pas : la page est celle d'avant,
au pixel. Écrit dans ce sens et pas dans l'autre, parce qu'une porte qu'on
n'ouvrirait pas resterait en travers de la page pendant un écran et demi — on
aurait remplacé une déception par une panne.

**Elle ne cache jamais rien** : `aria-hidden`, aucun contenu, aucun élément
focalisable, et les battants ne prennent pas le clic. C'est la règle du §8 bis
appliquée telle quelle.

Le prix est **un écran de défilement de plus** avant le verset du jour, sur la
page qui porte la conversion. Il tient dans un seul nombre — la hauteur de la
scène, `200lvh`. S'il pèse, elle descend.

#### Cinq choses apprises en le faisant, et aucune ne se devine

- **`lvh`, et c'est l'inverse de la règle du site.** Partout ailleurs c'est
  `dvh` : un bloc en `vh` saute quand Safari rétracte sa barre. Sur une scène
  épinglée, c'est `dvh` qui fait l'à-coup — la hauteur change *pendant* le
  geste, donc la course de l'animation change avec elle. `lvh` est constant.
  La règle du site borne un contenu ; celle-ci borne une course.
- **La perspective se prend en part d'écran**, comme `horizon` et le wordmark.
  À 1100 px en dur, le battant tourne franchement sur un ordinateur et n'est
  qu'un rectangle qui rétrécit sur un téléphone de 390 px.
- **Un pourcentage de `mask-position` ne mesure pas la boîte**, il aligne
  l'image : `100 %` veut dire « bord droit contre bord droit ». Le calage de la
  gravure partait donc de plusieurs centaines de pixels — d'un seul côté, ce
  qui ressemble à un dessin raté et non à une formule mal lue. La gravure est
  maintenant posée sur un pseudo-élément large comme la porte, où `center` est
  vrai sans calcul.
- **Une gravure se borne par la porte ; un horizon la déborde.** Le `max()`
  d'`horizon` recopié ici donnait 776 px de montagne dans une porte de 390 :
  les deux vantaux n'en montraient que la moitié centrale. `min()`, donc, et
  les deux termes changent de rôle.
- **Une arête doit être étroite et franche.** Posée large et pâle — 18 % de la
  largeur à 16 % d'or — il n'en restait sept pour cent sous l'opacité générale.
  Les battants se confondaient avec le fond dès la mi-course, et la seconde
  moitié du défilement ne montrait plus rien.
- **L'avancée se règle en part de la focale.** Un plan tiré vers l'œil grossit
  de `focale / (focale − avancée)`. Avec 240 px en dur contre une focale qui,
  elle, suit l'écran, ce rapport valait 1,28 sur un ordinateur et **1,85** sur
  un téléphone : porte grande ouverte, un battant vu de tranche couvrait encore
  cinquante points de bord et le chiffre romain de la section passait dessous.
  Un ornement qui mange le contenu qu'il vient de dévoiler. Les deux termes
  suivent maintenant le même rapport, 0,218.
- **La rotation seule ne libère pas le cadre.** À 82°, un battant garde le
  neuvième de sa largeur. Il finit donc sa course **hors de l'écran**, tiré vers
  l'extérieur — et ce dégagement est au **cube** : linéaire, la porte glisse
  autant qu'elle pivote et cesse d'être une porte ; au cube, il ne vaut rien
  avant les deux tiers de la course, puis l'emporte sur la fin.
- **La caméra arrive à 1, elle ne le dépasse pas.** Le premier réglage allait
  de 1 à 1,16 : le travelling finissait sur un verset seize pour cent trop
  gros, dans une échelle typographique dont chaque palier a été mesuré. Elle va
  de 0,88 à 1 — c'est le début qui recule.

#### Le portail — le dessin retenu

**Quatre dessins ont été comparés le 16 août 2026, et c'est celui-ci qu'il a
retenu.** Les trois écartés — `montagne`, la marque gravée à cheval sur la
fente ; `nus`, la `voute` pour seule face ; `voile`, deux pans qui se
dissipent — vivent dans le commit « Comparer quatre seuils avant d'en retenir
un ». Ne pas les réinventer sans l'avoir lu : chacun a été mené jusqu'au bout
et écarté en connaissance de cause.

Le reproche qui a produit celui-ci vaut pour les trois autres : la porte
occupait **tout l'écran**, bord à bord. Elle se lisait donc comme un mur qui se
fend. On ne voyait jamais la porte *en entier*, donc jamais qu'il y en avait
une. C'est le reproche de sa femme sur le bouton, un cran plus loin.

Le portail la pose au milieu — arche en demi-cercle, la nuit franche autour, la
lumière qui sort de la fente en éventail — et l'on **la franchit** : le mur
grandit et sort du cadre au lieu de s'estomper. Un mur qui s'efface se
remarque ; un mur qu'on dépasse, non.

Deux choix tranchés avec lui : le haut **arrondi** et non en pointe — l'arc
brisé est plus spectaculaire mais date d'un siècle précis, et le site ne date
de rien — et **des rais** en plus de la fente.

Rien de doré, rien de nuageux : la grandeur vient de la proportion et de la
lumière. Un portail chargé ferait basculer la page chez l'antiquaire en trois
secondes.

**Et pas de marque dessus.** La montagne y était gravée au premier jet ; il l'a
fait retirer, et il a raison. Le portail porte déjà son signe : c'est l'arche.
Un second signe posé dessus les met en concurrence, et le dessin cesse d'être
une porte pour redevenir l'image d'une porte.

##### Le premier jet était un contour, et il faisait de la peine

Il a regardé et il a dit « la porte fait toujours de la peine ». Il avait
raison, et ce n'était pas une question de goût : ce qui restait à l'écran était
un rectangle arrondi cerné d'un trait presque invisible, avec deux traits
dedans. Tout à moins de trois valeurs du fond.

On avait écrit ici même « la grandeur vient de la proportion et de la
lumière », puis livré la proportion seule. **Une règle qu'on énonce sans la
tenir est pire qu'une règle absente** : elle donne le sentiment d'avoir décidé.

Quatre manques, et ils sont la carte du bloc `.seuil--portail` :

1. **aucune lumière** — le joint était mort, alors que c'est de là que vient
   toute la promesse ;
2. **aucune matière** — un aplat `#261016` sur un aplat `#18090D` ;
3. **aucun poids** — rien ne disait que la porte était *posée dans* un mur ;
4. **un champ vide** au-dessus des battants, la moitié de l'arche pour rien.

La réponse : un **tympan** et un **linteau** — les battants deviennent droits,
la structure d'un vrai portail —, un **panneau creusé** par battant, une
embrasure qui s'**enfonce** par deux ombres intérieures, et un **joint d'or**
qui brûle dès la porte close.

Quatre leçons de plus, et deux sont des bugs :

- **`preserve-3d` était le mauvais outil, et il l'aurait cassé deux fois.**
  Avec un linteau, le mur ne peut plus grandir sans les battants : il faut les
  envelopper. Le réflexe est de poser `transform-style: preserve-3d` sur
  l'enveloppe pour que la perspective la traverse — elle ne traverse que si le
  parent est lui-même `preserve-3d`, or l'étage est plat. Les battants se
  seraient retrouvés sans perspective du tout, à tourner en projection
  orthographique. Et `preserve-3d` aurait coûté plus cher encore : les battants
  avancent en `translateZ`, donc ils seraient passés **devant** le mur, l'ordre
  des plans venant alors de la profondeur et non de `z-index`. La solution est
  de **descendre la perspective d'un cran**, sur l'enveloppe : les battants la
  reçoivent en enfants directs, tout reste plat vis-à-vis de l'étage, et
  `z-index` gouverne à nouveau.
- **Les deux couches d'ombre étaient à l'envers, sur les quatre dessins.** Un
  dégradé pose sa première butée à l'**opposé** de sa direction : dans
  `linear-gradient(to right, …)`, le `0 %` est au bord gauche. Écrites
  `to var(--vers-fente)` avec l'or en tête, elles mettaient l'or côté charnière
  et l'ombre côté fente — l'inverse exact de ce que le commentaire annonçait.
  Ça n'a pas sauté aux yeux, et c'est le pire : aux valeurs d'avant, rien
  n'était assez visible pour qu'on voie que c'était visible du mauvais côté.
  Un défaut invisible dans une pièce invisible.
- **Deux panneaux par battant dessinaient une croix.** Avec le joint vertical
  au milieu, les quatre panneaux la formaient en travers de la porte d'entrée —
  et l'ONT est une restitution du corpus hébreu antique. Ce n'est pas une
  question de goût : c'est une affirmation que l'auteur n'a pas faite, posée
  sur le premier écran. Un seul panneau haut ne laisse que le joint vertical,
  et c'est d'ailleurs le dessin des portes de nef.
- **Un joint est étroit, ou il n'est pas.** Étalé sur les deux tiers du
  battant, ce n'était plus un joint mais une bande peinte au milieu de la
  porte : la lumière ne venait plus de derrière, elle était *sur* le bois. Il
  tient dans les trois derniers centièmes. Et il **se retire** sur la fin —
  laissé plein, il restait posé sur le verset une fois la porte franchie,
  comme un projecteur sur du texte.
- **Le linteau restait en travers du verset, et seulement sur grand écran.**
  L'arche naît à `7 % + portail / 2` du haut, l'axe de la caméra est à 46 % :
  sur une fenêtre d'ordinateur, où le portail vaut `62lvh`, ces deux valeurs se
  touchent presque. Le linteau est donc **sur l'axe**, et un point sur l'axe ne
  s'écarte pas. Il faut alors une échelle de **5,75** pour le sortir par le
  haut, là où **1,93** suffit sur un téléphone — et le carré n'en donnait que
  3,6. Défaut strictement invisible au simulateur, puisqu'il ne se produit que
  là où le portail est proportionnellement le plus grand.

  L'échelle passe donc à la **puissance quatre**, coefficient 6 : rien avant
  les deux tiers — la porte tient, on l'approche —, puis tout à la fin. C'est
  aussi ce que fait un objet qu'on franchit vraiment : il ne grandit pas
  régulièrement, il passe.

  Le chiffre est **calculé et non réglé à l'œil** : `axe / (axe − linteau)`
  vaut 5,75 sur tous les formats d'ordinateur essayés, de 1024×768 à
  1728×1117. Sept laisse 22 % de marge, et un effacement sur les derniers
  centièmes couvre le format qu'on n'a pas essayé. C'est la seule façon de
  vérifier quoi que ce soit sur une géométrie qu'aucun de nos deux outils de
  capture ne sait montrer.
- **Une pierre qui n'a pas de trace d'outil est un aplat.** Le linteau est
  passé de 5 à 8,5 % de la largeur du portail, il **déborde** de l'embrasure
  de 2,6 % — un linteau qui affleure est un trait, un linteau qui avance est
  une pierre — et il porte des stries.

  Ces stries sont **deux périodes premières entre elles**, 23 et 31 pixels. Une
  seule, régulière, ne se lit pas comme une taille mais comme une grille :
  l'œil trouve le pas en une seconde et voit une clôture. Deux qui ne
  retombent jamais ensemble donnent une irrégularité qu'on ne peut pas suivre,
  et c'est exactement ce qu'on demande à une matière.
- **`inset 0 1px 0` ne suit pas une courbe.** Le tympan portait un filet d'or
  ainsi posé : sur un demi-cercle, il ne laisse qu'une écharde claire sur un
  seul flanc — qu'on prend ensuite pour un défaut de rendu et qu'on va
  chercher dans les ombres. Un seul trait par arête, et c'est l'arche qui le
  porte.
- **Le défilement du clic doit être linéaire, et ce n'est pas un goût.** Une
  cubique en S était en place, au motif qu'une vitesse constante « se lit comme
  un mécanisme ». C'est vrai d'une animation qu'on regarde ; ça ne l'est pas
  ici, où la porte n'est pas animée mais **pilotée par la position**. Toute
  inflexion de la vitesse du défilement devenait donc une inflexion de
  l'ouverture — la porte ne bougeait presque pas au départ, se précipitait au
  milieu, rampait à la fin. Adoucir un défilement et adoucir une porte sont
  deux décisions distinctes ; les superposer compose deux courbes dont on n'a
  choisi ni l'une ni l'autre. La courbe de la porte est dans la feuille ; le
  défilement ne fait que dérouler le temps.
- **Un dégradé répété ne fera jamais une matière.** Deux essais l'ont montré :
  des stries à 8 %, qui donnaient une grille ; puis deux périodes premières
  entre elles à 5 %, qui ne donnaient rien. Le problème n'est pas le réglage,
  c'est l'outil — une matière est irrégulière **à toutes les échelles**, et un
  dégradé ne l'est à aucune. Le `feTurbulence` de `grain-page` est le bon
  outil, et il était deux cents lignes plus haut.
- **Un grain gris désature tout ce qu'il touche.** Mesuré au pixel : en
  `overlay`, la table du linteau tombait à `#5C3D3D` — teinte 0° — et en
  `soft-light`, cru plus sûr, à `#5A3B3A`, saturation de 41 % à 21 %, quand
  toute la peau du site tient 343°. Ce n'est pas un réglage à corriger, c'est
  mécanique : les deux fusions passent la base par une racine ou un produit
  **canal par canal**, et sur une couleur sombre le canal le plus faible gagne
  le plus en proportion — le vert de l'aubergine monte trois fois quand le
  rouge monte deux, et la teinte tourne vers l'orange.

  Le grain est donc **teinté à la source** : un second `feColorMatrix` le mappe
  sur la rampe et lui donne une opacité constante, sa moyenne calée sur
  l'aubergine exactement. Il se pose alors sans fusion, et le résultat est le
  mélange de deux couleurs de la rampe — donc sur la rampe par construction. Il
  n'y a plus rien à vérifier.

  **L'œil n'aurait rien dit** : le linteau paraissait juste, et c'est la
  pipette qui a tranché. Sur une rampe dérivée au calcul, une dérive de teinte
  se mesure — elle ne se regarde pas.
- **C'est la taille du grain qui compte, pas sa force.** À la fréquence voisine
  de celle de `grain-page`, les grains font moins d'un pixel : au pixel près ils
  sont là, à bout de bras ils ne se lisent pas comme de la pierre mais comme un
  flou. On monte alors l'amplitude, et l'on obtient un flou plus sale. À 0,34 —
  quatre fois plus gros — avec trois octaves, ils ont de la structure à
  plusieurs échelles, et c'est ça qui distingue une matière d'un bruit.
  `grain-page` cherche exactement l'inverse : casser des bandes sans jamais se
  voir. Même filtre, deux réglages opposés, deux raisons opposées.
- **Une moulure est faite de faces, pas d'un dégradé.** Le linteau était un
  dégradé continu et c'est pour ça qu'il ne pesait rien : une pierre moulurée
  n'a pas de valeur qui glisse, elle a des plans qui se coupent. Cinq faces à
  butées franches, qui parcourent la rampe de bout en bout — l'arête d'or, la
  table, le congé, le listel rentrant, le dessous que rien n'éclaire jamais.
  Chaque butée est une arête, et c'est l'arête qui attrape la lumière.
- **Un pixel de retrait suffit à casser un cercle.** Le tympan était en
  `inset: 1px` : ses rayons se lisant sur sa propre boîte — 50 % de sa largeur,
  100 % de sa hauteur —, il perdait deux pixels de large et zéro de haut, donc
  son rayon horizontal cessait d'égaler le vertical. Sa courbe croisait celle
  de l'arche. Une ellipse déguisée en cercle ne se voit pas ; son intersection
  avec un vrai cercle, si.

**Aucun nouvel actif.** L'arche est un `border-radius`, le mur une ombre
portée, les rais un dégradé conique. Rien à charger, rien à empreindre.

Cinq choses apprises, et aucune ne se devine :

- **Le mur se pose par-dessus les battants, jamais autour.** L'envie naturelle
  est de découper les battants à la forme de l'arche, avec un `overflow: hidden`
  entre l'étape et eux. Ça ne peut pas marcher : `perspective` ne s'applique
  qu'aux enfants **directs**, il faudrait donc `transform-style: preserve-3d`
  sur ce conteneur — or un `overflow` autre que `visible` force le rendu à plat
  sur un parent `preserve-3d`. On perdrait la 3D pour gagner une découpe. Une
  ombre portée de `100vmax` peint la nuit tout autour du mur et ne dessine rien
  dedans ; les battants n'existent que dans le trou. Et c'est plus juste : un
  battant qui s'ouvre disparaît **derrière l'embrasure**, ce que fait une porte
  dans son tableau, et le mur y gagne une épaisseur qu'on n'a pas dessinée.
- **Le demi-cercle force l'unité.** Un pourcentage de `border-radius` se mesure
  sur la boîte elle-même — horizontalement sur sa largeur, verticalement sur sa
  hauteur. Le rayon vertical ne peut donc pas s'écrire en pourcentage : la porte
  est trois fois plus haute que large et l'arche s'aplatirait d'autant. La
  largeur se donne en unités de fenêtre, et la même valeur sert aux deux.
- **Ce qui fait la porte, c'est ce qu'il y a autour d'elle.** Au premier essai
  elle occupait 86 % de la largeur : il restait vingt-huit points de mur de
  chaque côté, et l'on ne lisait plus une porte dans un mur mais un écran aux
  coins arrondis.
- **Elle pose au sol et laisse du ciel.** Sans retrait haut, la clef de l'arche
  tombe exactement sur le bord de l'écran : la courbe y meurt au lieu de se
  fermer, et l'arche a l'air rognée.
- **Le franchissement est au carré, les rais en parabole.** Linéairement,
  l'arche avait quitté l'écran à 40 % de la course — on voyait le portail, puis
  plus rien, et les trois cinquièmes du défilement se jouaient sans lui. Les
  rais, eux, n'existent **que pendant le passage** : nuls porte close, nuls une
  fois dedans, au plus fort à mi-course. Des rais qui subsistent sur le verset
  une fois la porte franchie ne sont plus une lumière, ce sont des rayures sur
  du texte.

#### Deux défilements qui se battent, et une porte coupée

**Corrigé le 16 août 2026**, sur un enregistrement d'écran qu'il a fourni — et
les deux causes étaient invisibles au simulateur.

**`window.scrollTo(x, y)` hérite du `scroll-behavior` de la feuille**, et `html`
porte `smooth`. La traversée appelait donc, soixante fois par seconde, une
fonction qui **relance un défilement doux du navigateur** vers la position
demandée. Deux animations sur la même page, chacune corrigeant l'autre.

Ça ne se lit pas comme un ralenti, ça se lit comme des à-coups — et l'on cherche
la latence du côté du rendu, où il n'y avait rien. La vidéo l'a tranché :
mesurée image par image à 120 i/s, la traversée perdait des images et sautait,
avec des écarts où l'écran change presque entièrement d'une image à l'autre.
`behavior: instant` coupe la seconde animation ; la position demandée redevient
la position obtenue.

**`lvh` était le mauvais des deux unités constantes.** Il vaut la fenêtre barre
d'adresse **rétractée** — or la barre est là au chargement. La scène débordait
donc d'une centaine de points sous la barre d'outils, et le pied de la porte s'y
trouvait coupé, tympan poussé d'autant vers le bas. `svh` vaut la fenêtre barre
visible : la scène tient toujours. Ce qu'on paie est une bande de nuit sous la
porte quand la barre se rétracte — invisible, le mur du portail étant déjà de la
nuit sur un fond de nuit. **Couvrir l'écran valait moins que ne jamais être
coupé.**

**Et la cible d'arrivée devenait fausse en plein vol.** Safari rétracte sa barre
dès qu'on défile ; le Hero est en `dvh` — la règle du site — donc il *grandit* à
ce moment-là, et tout ce qui est en dessous descend d'une centaine de points.
Une cible calculée une fois au clic tombait alors trop haut, et la porte
s'arrêtait avant d'être ouverte. Elle est **relue à chaque image** : le
mouvement reste linéaire, c'est la destination qui bouge, et il se corrige au
lieu de se casser.

#### La latence, et la règle qui en sort

**Corrigé le 16 août 2026.** Il l'a dit en trois mots — « y a de la latence de
ouf » — et la cause était entièrement de notre fait.

**Une animation au défilement ne reste fluide que si elle ne pilote que
`transform` et `opacity`.** Ce sont les deux seules propriétés qu'un navigateur
sait animer sur le compositeur ; toute autre le ramène sur le fil principal —
celui qui traite aussi le défilement. On ne perd donc pas quelques images, on
perd le geste.

Quatre postes, du plus cher au moins cher :

- **Le grain de pierre se rejouait à chaque image.** Le SVG n'avait pas de
  taille propre, donc le navigateur l'étirait à la boîte — et **rejouait le
  `feTurbulence`** à chaque changement d'échelle. Trois octaves, soixante fois
  par seconde, sur quatre éléments et en double passe. Il porte maintenant une
  taille — 168 px — et se répète : le bruit est calculé une fois. Le défaut
  n'apparaît dans aucun profil de style ; il est dans la rastérisation.
- **Le fond porte le bloc du verset, donc du texte**, et il est mis à
  l'échelle. Sans `will-change: transform`, le navigateur re-rastérise tout le
  texte à chaque image pour le garder net. Promu, il est rendu une fois et la
  couche glisse. Douze pour cent d'agrandissement, qu'aucun œil ne mesure.
- **Une largeur qui change est une mise en page qui recommence.** La lueur
  s'écrivait `width: calc(3% + ouverture * 86%)`, avec son dégradé à repeindre
  derrière. Largeur figée, `scaleX()` à la place.
- **Une couleur calculée par image est un repeint par image.** Le filet de
  l'arche et le joint interpolaient un `color-mix` dont le pourcentage
  dépendait de `--ouverture`. Le gain visuel ne valait pas la dépense :
  l'arche est déjà éclairée par ce qui passe derrière elle.

Et le `voile` gardait la seule animation de `filter` du lot — un flou gaussien
recalculé sur un pan de la hauteur de l'écran, à chaque image. Il est
**constant** maintenant : ce qu'on perd est le passage du net au flou, ce
qu'on garde est l'essentiel — une étoffe n'a pas d'arête, même immobile.

**La vérification est mécanique**, et c'est ce qui la rend utile : relever
toutes les propriétés dont la valeur contient `var(--ouverture)`, et vérifier
qu'il n'y a que `transform` et `opacity`. Une propriété de plus dans cette
liste, et la scène retombe sur le fil principal.

#### La couture avec l'ouverture

**Corrigé le 16 août 2026.** Une coupure nette se voyait à la jonction : le
Hero finit sur de l'aubergine plein — le bas de sa `voute` — et la scène
commençait sur une autre matière. `portail` était le pire des quatre, son mur
étant de la nuit franche : deux valeurs aux deux bouts de la rampe, bord à
bord.

Ailleurs sur le site, ce saut existe aussi et il est **assumé** : un `Bloc`
porte un filet en haut, qui dit « nouvelle section ». Le seuil n'en est pas
une — c'est le lieu où l'on entre, la suite de l'ouverture. Un filet y
mentirait, et une coupure sans filet se lit comme un défaut.

On prolonge donc la lueur du Hero au lieu de la couper : **son ellipse,
retournée**, posée au-dessus de tout, mur compris.

Et c'est bien la sienne, à la valeur près — `ellipse 120% 70%` sur toute la
hauteur de l'étape, ancrée en haut là où `voute` l'ancre en bas. Les deux
courbes sont alors le miroir l'une de l'autre et se rejoignent à la même valeur
**en tout point de la couture**, pas seulement au milieu.

Le premier essai avait redessiné une ellipse à l'œil, plus courte et plus
large. Elle rattrapait le centre et laissait un liséré **aux deux bords** —
c'est-à-dire là où personne ne pense à regarder. La leçon est celle du §5 sur
l'échelle typographique : deux valeurs qui doivent s'accorder ne s'accordent
pas à l'œil sur un point, elles s'accordent par construction sur tout le
domaine.

Elle s'éteint à mesure qu'on ouvre, et ce n'est pas une économie : passé
l'épinglage, le Hero est hors de l'écran et il n'y a plus de couture à
masquer. Un voile d'aubergine posé sur le verset ne serait plus qu'un voile
d'aubergine posé sur le verset.

#### Ce qu'a coûté la comparaison, et ce qu'elle a appris

Quatre dessins menés de front demandent un banc d'essai : `?porte=` choisissait
le dessin, `?ouverture=` figeait la scène à une progression donnée pour la
photographier, `scripts/portes.sh` en tirait une planche de seize images. Tout
ça est **supprimé** avec les trois écartés — c'était l'instrument d'un choix,
pas une fonctionnalité. Deux leçons en restent, et elles resserviront.

**Une porte au défilement ne se juge pas sur une capture, mais une porte figée,
si.** Encore fallait-il la poser **par-dessus la page** : les deux outils de
capture ne voient que le premier écran — QuickLook rend une vignette du haut,
le simulateur ne sait pas défiler. Laissée à sa place dans le flux, sous une
ouverture qui fait déjà un écran, elle n'entrait dans aucun des deux. Quatre
aperçus sont sortis identiques à l'octet près, et l'on a cherché le défaut dans
le dessin.

**Un outil qui imite le produit doit porter sa marque.** Le mode figé posait la
scène en `position: fixed` et coupait son animation : la porte ne bougeait
plus, et le reste glissait derrière. C'est exactement ce qu'on demande à un
banc d'essai — et c'est indiscernable d'une porte cassée. Il a filmé dix
secondes d'un défaut qui n'existait pas, sur une URL donnée sans étiquette. Le
diagnostic a tenu à une image : juste après un rechargement, à défilement zéro,
on voyait la porte au lieu du Hero. Aucun mode vivant ne fait ça.

### Le massif se dimensionne en part d'écran, pas en largeur

**Corrigé le 14 août 2026.** Les massifs de l'ouverture étaient dimensionnés à
la seule largeur — 165 % de l'écran. Un masque garde ses proportions : sa
hauteur vaut donc la moitié de sa largeur. Sur une fenêtre d'ordinateur, ces
165 % donnent près de 90 % de la hauteur, la crête monte derrière le titre, et
l'on est **dans** un lieu. Sur un téléphone, les mêmes 165 % ne donnent plus
que 37 % de la hauteur : la montagne se tasse en bas, on n'en voit qu'un bout
de crête, et elle redevient l'autocollant qu'on voulait éviter.

L'utilitaire `horizon` prend donc la largeur au plus grand de deux termes — la
part de largeur, ou la même part convertie en hauteur. Un `max()` et non une
requête média : le seuil n'est pas une largeur, c'est le **rapport** de la
fenêtre, et il bascule tout seul. Une fenêtre en paysage garde exactement la
composition d'avant.

Trois choses à ne pas refaire :

- **Le coefficient est 0,904, pas la parité géométrique (1,067).** À parité, la
  montagne occupe la même part de hauteur que sur un ordinateur — 87 % — et il
  ne reste plus de fond au-dessus de sa crête. Comme le massif est aubergine
  sur une nuit d'aubergine, l'écran devient une masse unie : on ne voit plus une
  montagne, on voit une couleur. Ce qui fait la montagne n'est pas sa masse,
  c'est **son bord**, et un bord a besoin de ciel. Chiffre trouvé en regardant
  trois tailles côte à côte au simulateur ; le calcul donnait la bonne géométrie
  et la mauvaise image.
- **`horizon` est séparé de `massif`, et il le faut.** `massif` sert aussi de
  petit signe de quelques pixels dans un sommaire ou une carte, où c'est un
  `w-7` qui commande. Une largeur dans `massif` et une classe `w-*` sur le même
  élément, c'est le piège de `Bloc` rejoué — à spécificité égale, c'est l'ordre
  de la feuille qui tranche.
- **La règle vit en CSS, pas en classe Tailwind.** Elle porte deux `width` de
  suite : la seconde écrase la première là où `max()` est compris, la première
  tient partout ailleurs. Une classe ne porte qu'une déclaration, donc pas de
  repli — et sans repli la montagne ne serait pas *approximative*, elle serait
  **absente**. Une largeur nulle ne casse rien : la page s'affiche, se lit, et
  le décor manque sans que rien ne le dise.

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
| `icone-192.png`, `icone-512.png` | les icônes que réclame le manifeste Android |
| `icone-masquable-512.png` | la même, montagne à 50 % — voir ci-dessous |
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

Trois outils, et ils ne servent pas à la même chose.

`scripts/apercu.py` rend une page avec QuickLook, qui embarque WebKit. Il suffit
à juger une composition sur grand écran.

`scripts/verifier-composition.py` lit les douze gabarits **rendus** et échoue
s'il trouve une ponctuation double qui peut tomber à la ligne. À lancer après
toute page écrite — voir §8 bis.

**Avant toute vérification visuelle : `./scripts/dev-sync-empreintes.sh`.** En
mode `watch`, cargo-leptos régénère `target/site/pkg/ontbible.{css,js,wasm}`
mais **pas** les copies empreintées — et c'est celles-là que le serveur écrit
dans le HTML, puisqu'il démarre avec `target/debug/hash.txt`. La page servie
porte donc les assets du dernier **redémarrage complet**, pas ceux du dernier
enregistrement. On corrige un jeton, on recharge, rien ne bouge, et l'on croit
que la correction est fausse. Une heure a été perdue là-dessus, à débattre d'un
rendu qui n'était pas celui du code.

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

**La limite des masques est levée.** QuickLook refuse un masque CSS qui pointe
un SVG **par son chemin** — le signe de section et les massifs arrivaient en
blancs, et l'on avait pris ça pour une limite de l'outil. C'en était une du
chemin, pas du masque : le même fichier **en `data:`** est rendu. `apercu.py`
l'embarque donc dans sa feuille d'aperçu, et une ouverture se juge enfin sur
grand écran plutôt qu'au seul simulateur, qui ne montre qu'un téléphone.

La feuille du site, elle, garde son chemin : un SVG recopié en base64 dans la
CSS de production la gonflerait et l'empêcherait d'être mise en cache à part.

### Et le navigateur gardait quand même l'ancienne feuille

**Corrigé le 16 août 2026**, et c'est le troisième piège de cette section — le
plus coûteux, parce qu'il fait mesurer faux au lieu de ne rien montrer.

`dev-sync-empreintes.sh` recopie bien la feuille fraîche sur son nom empreinté.
Mais le **nom ne change pas** : `hash.txt` n'est recalculé qu'à une
construction complète. Le serveur ne posait alors aucune politique sur `/pkg/`,
donc Safari appliquait son cache heuristique — et resservait sa copie sans même
revalider.

La page capturée au simulateur portait le style d'il y a une heure. On mesure
un décalage de vingt-quatre points, on cherche la cause dans la règle qu'on
vient d'écrire, on la réécrit trois fois, on va jusqu'à retirer la
transformation pour voir — et la règle n'était jamais arrivée. C'était l'ancien
grossissement de caméra, `1 + o × 0,16`, qui donne exactement ce décalage.

C'est le §8 ter à l'envers : les empreintes protègent la production parce que
le nom change avec le contenu, et en développement il ne change pas.

Le serveur déclare donc **`no-store` sur tout** quand il est compilé en debug
(`main.rs::sans_cache`). `no-store` et non `no-cache` : on ne veut pas d'une
révalidation, on veut qu'il n'y ait rien à révalider — un `304` sur une feuille
dont le nom n'a pas bougé rendrait exactement l'ancienne. La production n'est
pas touchée : CloudFront envoie `/pkg/` au seau, jamais à la Lambda.

**Le symptôme à reconnaître** : une correction de style qui « ne fait rien » au
simulateur alors que `target/site/pkg/ontbible.css` la contient. Vérifier
d'abord `curl -sI …/pkg/ontbible.<empreinte>.css`, pas la règle.

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
/manifest.webmanifest              ce qui rend le site installable sur Android
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

**Et elle ne s'appliquait qu'à la moitié du site. Corrigé le 14 août 2026.**
`composer` ne voyait que `Noeud::Texte`, donc ce qui traverse l'arbre. Trois
familles de textes lui échappaient, et chacune se coupait à la ligne :

- la **prose du site**, écrite en littéraux Rust — 40 cas ;
- les littéraux **coupés en deux** par une continuation de ligne, qu'aucune
  recherche sur les sources ne rapproche : l'espace finit un littéral, la
  ponctuation commence le suivant ;
- les **chaînes nues du corpus** — le verset du jour, le rendu d'un
  intraduisible, l'extrait d'une occurrence. Vingt-trois coupures possibles sur
  la seule fiche d'`adam`.

`composer` est donc publique, et toute chaîne du corpus posée dans une page y
passe. La vérification, elle, se fait sur le **rendu** et pas sur les sources —
c'est la seule qui voie les trois familles à la fois :

    ./scripts/verifier-composition.py

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

## 8 quater bis. Les images portent une empreinte, elles aussi

**Corrigé le 14 août 2026.** Les feuilles et le WASM portent leur empreinte dans
le nom du fichier ; les images n'avaient jamais eu ce traitement. `wordmark.svg`
gardait son nom quand son dessin changeait, et il est servi avec un cache d'une
journée : **une image corrigée restait invisible jusqu'à vingt-quatre heures**
pour qui avait déjà visité le site.

Ce n'est pas théorique. Le ® a été retiré de la marque le 13 août au soir ; le
lendemain matin on en débattait encore, en regardant deux versions différentes
du même fichier — la production était juste depuis le début.

L'adresse porte donc `?v=<empreinte>`, calculée à la compilation par `build.rs`
sur le contenu du fichier. Le fichier garde son nom sur le seau ; seule
l'adresse change. Trois choses à savoir :

- **Un paramètre et non un nom.** Renommer demanderait de recopier les fichiers
  au build, alors que `public/` est posé tel quel sur le seau.
- **Une empreinte maison, FNV-1a.** Ce qu'on demande à cette valeur, c'est de
  changer quand le contenu change. Une collision ne serait pas une faille,
  seulement une image périmée de plus — ce qu'on avait déjà sur toutes.
- **L'invalidation couvre `/images/*`.** Le paramètre suffit au navigateur, qui
  y voit une autre ressource. Mais la politique de cache du bord n'entre pas les
  paramètres dans sa clé : sans l'invalidation, CloudFront servirait l'ancienne
  image pendant une journée, à tout le monde. Les deux couches demandent chacune
  leur moyen.

Toute image de `public/images/` passe par `design::image()`, et un test exige
que le dossier entier soit couvert : une image ajoutée entre dans la table sans
que personne n'ait à y penser.

## 8 quinquies. Le déploiement continu

Deux workflows, et ils ne se ressemblent pas.

`.github/workflows/eprouver.yml` — à chaque pull request vers `main`. Il
n'existe que parce que `deployer.yml` faisait tourner les tests **après** la
fusion, dans le job qui déploie : la séquence était fusionner, puis découvrir.
Sur un dépôt où `main` part en ligne toute seule, c'est le mauvais ordre.

Il refait les mêmes étapes jusqu'à la construction, et rien de ce qui vient
après. Il ne peut pas déployer, et ce n'est pas une promesse : `infra/ci.tf`
épingle la condition OIDC à `ref:refs/heads/main`, or le jeton d'une pull
request porte `ref:refs/pull/<n>/merge`. AWS refuse le rôle. D'où l'absence de
`permissions: id-token` — on ne demande pas un jeton dont on n'a aucun usage.

Il lance le serveur et lui pose de vraies questions : cinq routes, puis
`verifier-composition.py`. Un binaire qui compile et ne démarre pas reste un
binaire cassé, et c'est le genre de panne que la Lambda annonce par un
`Runtime.InvalidEntrypoint` sans un mot de plus.

**Ce qu'aucun des deux n'attrape** : ce qui est juste mais faux. Une échelle
typographique qui s'inverse sous 1024 px compile, passe les 66 tests, et se
déploie. Il a fallu un simulateur et une capture — voir §7 bis.

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

### Le site s'installe sur Android

`public/manifest.webmanifest` lui donne un nom, une icône, la nuit d'aubergine
et `display: standalone` — Chrome propose alors « Installer l'application », et
le site s'ouvre en plein écran, sans barre de navigateur. Sans manifeste, la
même commande ne pose qu'un raccourci sans nom propre, ouvert dans un onglet.

Il est servi par une **route**, comme le fichier d'association, et pas depuis
`public/`. La raison est le déploiement : le paquet Lambda ne porte que le
binaire et `hash.txt`, et CloudFront n'envoie au seau que `/pkg/`, `/images/`,
`/fontes/` et `robots.txt`. Un fichier posé ailleurs dans `public/` n'est donc
joignable **nulle part** en production — il marche en local, et c'est tout.
`include_str!` garde le fichier comme source, pour ne pas avoir deux endroits à
tenir d'accord.

**L'icône masquable est un fichier à part, et il le faut.** Android impose sa
propre forme — cercle, squircle, goutte selon le constructeur — en rognant ce
qui dépasse, et ne garantit que le cercle inscrit dans les 80 % centraux. La
montagne y passe donc à 50 % du côté au lieu de 72 %. Déclarer l'icône pleine
comme masquable la ferait tronquer partout : c'est le défaut le plus courant
des manifestes.

Ce que l'installation ne donne pas : la lecture hors ligne, le widget et les
notifications. Il n'y a **pas de service worker**, et c'est délibéré — un
worker qui met en cache sans qu'on l'ait pensé sert du HTML périmé pendant des
jours, exactement le défaut que les empreintes du §8 ter viennent de corriger.

## 9. Ce qui reste à trancher

- Le **texte de la page auteur** — le jet est écrit, il attend sa relecture.
  **Rien de cette page ne doit être mis en ligne avant.**
- **Jost ou EB Garamond** pour le corps du site. Jost est en place, EB Garamond
  est chargée à côté : un seul jeton à changer pour comparer en direct.
- Le **dépôt de la marque à l'INPI**. Apposer ® sur une marque non déposée
  relève de l'article L.716-9 du code de la propriété intellectuelle — ce
  n'est pas un risque commercial dont on pèserait la probabilité, c'est une
  infraction. Le sigle a donc été **retiré des vecteurs** le 13 août 2026
  (`scripts/retirer-le-r.py`), et les originaux attendent sous `*-avec-r.svg`.

  **Décidé le 14 août 2026** : une **marque verbale**, « La Bible ONT », et non
  le dessin. Elle protège le nom quelle que soit la typographie — c'est le nom
  que porte le ®, et un dépôt figuratif laisserait quelqu'un reprendre
  « La Bible ONT » dans une autre lettre sans rien enfreindre.

  Trois classes, et le choix se fait **maintenant** : on n'ajoute pas une
  classe à une marque déposée, il faut redéposer. Quarante euros aujourd'hui
  contre cent quatre-vingt-dix plus tard.

  | classe | ce qu'elle couvre |
  |---|---|
  | 9 | l'application, les publications électroniques téléchargeables |
  | 16 | les produits de l'imprimerie — l'édition papier qu'une Bible finit par avoir |
  | 41 | la publication en ligne, l'édition, la traduction |

  190 € la première classe, 40 € les suivantes : **270 €**.

  Le déposant est la **personne physique**, sous son identité légale — Yannis
  Bikouta. « Gloire » est le prénom public (§2), pas celui d'un acte.

  À faire avant de payer : la **recherche d'antériorité** sur `data.inpi.fr`,
  gratuite. Elle n'est pas une formalité — l'INPI n'examine pas les marques
  antérieures, il publie et laisse les titulaires s'opposer. Un dépôt qui
  heurte une marque existante est perdu, et les 190 € avec.

  Et le ® ne redevient légitime qu'à l'**enregistrement**, pas au dépôt :
  publication au BOPI sous six semaines, deux mois d'opposition, environ cinq
  mois en tout s'il n'y a ni objection ni opposition.

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
