#!/usr/bin/env python3

"""Engendre les QR codes de la page de l'application.

    ./scripts/qr-app.py            # pip install qrcode, si besoin

## Il y en a deux, et il en faut deux

`qr-app.svg` mène à la fiche App Store, `qr-beta.svg` à la bêta TestFlight. La
page n'en montre qu'un à la fois — celui de l'état où l'app se trouve — mais les
deux fichiers existent en permanence.

C'est délibéré. Réécrire `qr-app.svg` vers TestFlight le temps de la bêta
donnerait un fichier dont le nom ment sur le contenu, et le jour de la
publication personne ne se souviendrait qu'il faut le régénérer. Un QR faux ne
se voit pas : il se scanne, il ouvre quelque chose, et c'est la mauvaise chose.

## Pourquoi un QR et pas seulement un lien

La page de l'application se lit surtout sur un **grand écran** — c'est là qu'on
tombe sur le site, depuis un moteur ou un lien partagé. Or l'app s'installe sur
un téléphone. Le badge App Store, cliqué depuis un ordinateur, ouvre iTunes ou
une page web : il ne pose rien sur l'appareil qui compte.

Le QR fait le pont. On sort son téléphone, on vise, on installe.

Sur téléphone il n'a aucun sens — on ne scanne pas l'écran qu'on tient — et la
page ne l'affiche donc qu'à partir de `sm`.

## Un SVG, pas un PNG

Un QR est fait de formes géométriques : c'est exactement ce qu'un vecteur rend
le mieux, et ce qu'une image matricielle rend le moins bien. Un PNG mis à
l'échelle brouille les bords, et un lecteur qui hésite est un lecteur qui
repart.

## Le dessin : des points, et des repères **carrés**

Le premier jet posait un `<rect>` carré par module. Il se scannait très bien et
il était dur — une grille d'angles vifs, dans une page qui n'en a pas un seul,
à la taille d'une vignette.

Le dessin actuel est celui de la Bible App de YouVersion, relevé sur leur page
de téléchargement, parce qu'il résout le problème proprement :

- **Les modules de données sont des points séparés**, d'un rayon de 0,42
  module. Le filet de fond qui passe entre eux est ce qui donne la légèreté.
- **Les trois repères restent des carrés pleins**, aux angles vifs, exactement
  comme la norme les dessine. C'est le seul endroit du code où l'on ne touche
  à rien.
- **Un creux de neuf modules au centre** reçoit la montagne.

## Ce qui casse un QR, et ce qui ne le casse pas

Il a fallu une erreur pour le savoir, et elle mérite d'être écrite.

Le premier essai en points ne se décodait pas — zéro fois sur neuf. J'en avais
conclu que **réduire les modules** cassait la lecture, et j'étais passé à un
dessin timide : des carrés pleins aux angles adoucis. C'était faux. Dans ce
test, les repères étaient **eux aussi** dessinés en points, et je n'ai jamais
isolé les deux.

Une fois isolés, tout se renverse. Éprouvé sur les fichiers réellement
engendrés — rastérisés par QuickLook, décodés par OpenCV à neuf tailles de 512
à 120 px, chacune floutée :

| données | repères | décodages |
|---|---|---|
| carrés pleins | carrés | 9/9 |
| **points r = 0,36 à 0,48** | **carrés** | **9/9 à tous les rayons** |
| angles adoucis à 0,4 | adoucis | 7/9 |
| points r = 0,50 | arrondis | 7/9 |
| points r = 0,42 | **en points** | **0/9** |

La leçon tient en une ligne : **les données se dessinent comme on veut, les
repères ne se dessinent pas**. Ils sont ce qu'un lecteur cherche en premier, et
il les cherche à leurs proportions 1:1:3:1:1 — les arrondir, ou pire les
pointiller, lui retire son point de départ. Le reste du code, lui, est
reconstruit par correction d'erreur, et il en faut beaucoup pour l'empêcher.

Et la morale de méthode : un test qui fait varier deux choses à la fois ne
prouve rien sur l'une ni sur l'autre. La conclusion tirée du premier essai
était non seulement fausse, elle a fait livrer un dessin plus laid que
nécessaire.

## La correction d'erreur passe de `L` à `Q`

C'est un revirement, et l'ancienne raison était juste : un QR affiché sur une
dalle ne se salit pas, et monter le niveau ajoute des modules, donc réduit la
taille de chacun à surface égale.

Mais on **creuse** maintenant le centre, et un creux est une dégradation — la
seule différence avec une tache de café est qu'on la place soi-même.

| niveau | modules | tolérance | creux | le module à 144 px |
|---|---|---|---|---|
| `L` | 29 | 7 % | 8,4 % | 3,9 px |
| `M` | 29 | 15 % | 10,4 % | 3,9 px |
| **`Q`** | **33** | **25 %** | **6,7 %** | **3,5 px** |
| `H` | 37 | 30 % | 5,4 % | 3,2 px |

`L` est aussitôt disqualifié — le creux à lui seul dépasse sa tolérance.

L'habitude, sur un QR à logo, est de prendre `H`. Elle vient de logos qui
occupent quinze à vingt-cinq pour cent du code ; le nôtre en efface sept. `H`
paierait une marge dont on n'a pas l'usage, en modules dix pour cent plus
petits — et sur un code lu à trente centimètres par la caméra d'un téléphone,
c'est la taille du module qui décide.

`Q` laisse près de quatre fois plus de marge que le creux n'en consomme. Le
script le vérifie à chaque exécution et refuse d'écrire au-delà de 15 %.

## La montagne n'est pas dans ce fichier

Le creux la reçoit, la page l'y superpose — `signe-montagne`, un masque qui
pointe `logomark.svg`, déjà en cache et déjà aux couleurs du thème. L'inliner
ici pèserait neuf kilo-octets par QR, pour un dessin que le site charge de
toute façon.
"""

import sys
from pathlib import Path

try:
    import qrcode
except ModuleNotFoundError:
    raise SystemExit("Il manque `qrcode` : pip install qrcode")

IMAGES = Path(__file__).resolve().parent.parent / "public/images"

# La fiche, par son identifiant. Pas par un nom lisible : Apple les recompose
# quand le titre change, et l'identifiant, lui, ne bouge jamais.
#
# Le lien de bêta est celui du groupe externe « Beta » dans App Store Connect,
# onglet TestFlight → Public Link. Il ne change pas quand une version nouvelle
# est envoyée : c'est le groupe qu'il désigne, pas la version.
CIBLES = [
    ("qr-app.svg", "https://apps.apple.com/fr/app/id6801192372", "sur l'App Store"),
    ("qr-beta.svg", "https://testflight.apple.com/join/RAe4uzMu", "en bêta sur TestFlight"),
]

# Une marge de quatre modules est le « quiet zone » exigé par la norme : sans
# elle, un lecteur ne trouve pas les repères.
MARGE = 4

# Le côté d'un repère, fixé par la norme. Il ne se règle pas — et son dessin
# non plus, voir le préambule.
REPERE = 7

# Le creux central, en modules — impair, comme le côté d'un QR, sinon il ne
# tombe pas au centre. Neuf donnent à la montagne une trentaine de pixels sur un
# QR affiché à 144 ; sept ne lui en laissaient que vingt-quatre.
CREUX = 9

# Le rayon d'un point de données, en modules. De 0,36 à 0,48, tout décode neuf
# fois sur neuf : c'est donc un choix de dessin, pas de fiabilité. 0,42 laisse
# un filet de fond franc sans maigrir l'encre.
RAYON = 0.42

# Au-delà, le creux mange plus que la correction d'erreur ne peut rendre.
CREUX_MAXIMAL = 0.15


def engendrer(nom: str, cible: str, ou: str) -> None:
    qr = qrcode.QRCode(error_correction=qrcode.constants.ERROR_CORRECT_Q, border=MARGE)
    qr.add_data(cible)
    qr.make(fit=True)
    matrice = qr.get_matrix()
    cote = len(matrice)
    modules = cote - 2 * MARGE

    # Les trois repères, par le coin haut-gauche de chacun. Le quatrième coin
    # n'en porte pas : c'est lui qui donne au lecteur le sens de lecture.
    coins = sorted(
        {
            (MARGE, MARGE),
            (MARGE, MARGE + modules - REPERE),
            (MARGE + modules - REPERE, MARGE),
        }
    )

    def dans_un_repere(x: int, y: int) -> bool:
        return any(cx <= x < cx + REPERE and cy <= y < cy + REPERE for cx, cy in coins)

    debut = (cote - CREUX) // 2
    fin = debut + CREUX

    def dans_le_creux(x: int, y: int) -> bool:
        return debut <= x < fin and debut <= y < fin

    # Les données. Le module de rang `x` occupe `[x − 0,5, x + 0,5]` : la
    # viewBox est décalée d'un demi pour que les centres tombent sur des
    # entiers, ce qui allège le fichier d'un tiers.
    points = "".join(
        f'<circle cx="{x}" cy="{y}" r="{RAYON}"/>'
        for y in range(cote)
        for x in range(cote)
        if matrice[y][x] and not dans_un_repere(x, y) and not dans_le_creux(x, y)
    )

    # Les repères, en carrés francs. Le cadre est un seul tracé évidé en
    # `evenodd` — ça évite d'avoir à peindre un fond pour creuser le milieu, et
    # ça garde le fichier indifférent à la couleur de la page.
    yeux = "".join(
        f'<path fill-rule="evenodd" d="'
        f"M{cx - 0.5},{cy - 0.5} h{REPERE} v{REPERE} h-{REPERE} z "
        f'M{cx + 0.5},{cy + 0.5} h{REPERE - 2} v{REPERE - 2} h-{REPERE - 2} z"/>'
        f'<path d="M{cx + 1.5},{cy + 1.5} h3 v3 h-3 z"/>'
        for cx, cy in coins
    )

    # Ce qu'on a effacé, et que la correction d'erreur doit absorber. On le dit
    # à chaque exécution : le jour où l'adresse rallonge, le QR gagne des
    # modules et la proportion baisse — mais si elle montait, il faudrait le
    # voir ici plutôt que sur un téléphone qui ne scanne plus.
    efface = sum(1 for y in range(debut, fin) for x in range(debut, fin) if matrice[y][x])
    part = efface / max(1, sum(sum(ligne) for ligne in matrice))
    if part > CREUX_MAXIMAL:
        raise SystemExit(
            f"{nom} : le creux efface {part:.0%} des modules pleins, trop pour « Q »"
        )

    # `currentColor`, et **aucun fond** : c'est la page qui pose la pastille
    # claire derrière. Un QR a besoin d'un fond clair pour être lu, mais le
    # décider ici l'imposerait à tous les emplacements futurs.
    sortie = IMAGES / nom
    sortie.write_text(
        f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="-0.5 -0.5 {cote} {cote}" '
        f'role="img" aria-label="Code QR vers La Bible ONT {ou}">'
        f'<g fill="currentColor">{yeux}{points}</g></svg>\n'
    )
    print(
        f"  {nom:12} {modules}×{modules} modules  creux {part:.1%}  "
        f"{sortie.stat().st_size // 1024} Ko  → {cible}"
    )


def main() -> None:
    for nom, cible, ou in CIBLES:
        engendrer(nom, cible, ou)


if __name__ == "__main__":
    sys.exit(main())
