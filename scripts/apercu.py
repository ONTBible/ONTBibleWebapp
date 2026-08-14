#!/usr/bin/env python3

"""Rend des pages du site en image, via QuickLook.

## Pourquoi ce script existe

Il n'y a pas de navigateur dans l'environnement où ce site se construit. Sans
lui, on ne peut affirmer qu'une chose : « ça compile ». C'est très loin de
« ça se regarde », et la différence coûte un aller-retour à chaque fois.

QuickLook rend du HTML avec WebKit. Il suffit de lui donner une page dont les
chemins sont relatifs — le serveur les écrit en absolu — et de retirer les
scripts, qui ne servent pas à une capture.

    ./scripts/apercu.py /tmp/vues accueil=/fr auteur=/fr/l-auteur

Le serveur de développement doit tourner (`cargo leptos watch`).

## Ce qu'il ne montre pas

Le responsive. QuickLook rend à une fenêtre fixe et large, puis réduit
l'image : les requêtes média y voient toujours un grand écran. Pour un
téléphone, c'est `scripts/sim.sh` et rien d'autre.

Il montrait aussi les masques comme des blancs — le massif de l'ouverture, le
signe de section — et l'on avait pris ça pour une limite. C'en était une du
**chemin**, pas du masque : embarqué en `data:`, il est rendu. Voir plus bas.
"""

import base64
import pathlib
import re
import subprocess
import sys
import urllib.request

SERVEUR = "http://127.0.0.1:3000"
RACINE = pathlib.Path(__file__).resolve().parent.parent / "target" / "site"


def main() -> None:
    if len(sys.argv) < 3:
        raise SystemExit(__doc__)

    sortie = pathlib.Path(sys.argv[1])
    sortie.mkdir(parents=True, exist_ok=True)

    # La feuille porte une empreinte dans son nom — `ontbible.<empreinte>.css`,
    # voir le §8 ter — et cette empreinte change à chaque modification du style.
    # On la cherche donc au lieu de l'écrire : un nom en dur ne vaut que jusqu'à
    # la prochaine retouche de CSS.
    feuilles = sorted((RACINE / "pkg").glob("ontbible*.css"))
    feuilles = [f for f in feuilles if f.name != "apercu.css"]
    if not feuilles:
        raise SystemExit(
            "Aucune feuille dans target/site/pkg/. Lancez `cargo leptos watch`,\n"
            "et attendez qu'il ait fini de construire."
        )
    feuille = feuilles[0]

    # La feuille compilée pointe /fontes/ et /images/ depuis la racine du site ;
    # ouverte comme un fichier, elle les chercherait à la racine du disque.
    css = feuille.read_text()
    css = css.replace('url("/fontes/', 'url("../fontes/').replace(
        'url("/images/', 'url("../images/'
    )

    # Les masques rentrent, et c'est ce qui rend cet outil utile.
    #
    # QuickLook refuse un masque CSS qui pointe un SVG **par son chemin** —
    # le massif de l'ouverture et le signe de section arrivaient en blancs, et
    # l'on a longtemps pris ces vides pour une limite à contourner. Ce n'en est
    # pas une : le même masque **en `data:`** est rendu. Le fichier est donc
    # embarqué dans la feuille d'aperçu, et une ouverture se juge enfin ici
    # plutôt qu'au seul simulateur, qui ne montre qu'un téléphone.
    #
    # La feuille du site, elle, garde son chemin : un SVG de 4 Ko recopié en
    # base64 dans la CSS de production la gonflerait et l'empêcherait d'être
    # mise en cache à part.
    for nom in ("logomark",):
        source = RACINE.parent.parent / "public" / "images" / f"{nom}.svg"
        uri = "data:image/svg+xml;base64," + base64.b64encode(source.read_bytes()).decode()
        css = css.replace(f'url("../images/{nom}.svg")', f'url("{uri}")')

    (RACINE / "pkg/apercu.css").write_text(css)

    fichiers = []
    for argument in sys.argv[2:]:
        nom, chemin = argument.split("=", 1)
        html = urllib.request.urlopen(f"{SERVEUR}{chemin}").read().decode()
        html = re.sub(r'(src|srcset|href)="/', r'\1="', html)
        html = html.replace(f"pkg/{feuille.name}", "pkg/apercu.css")
        html = re.sub(r"<script.*?</script>", "", html, flags=re.S)
        fichier = RACINE / f"apercu-{nom}.html"
        fichier.write_text(html)
        fichiers.append(str(fichier))

    subprocess.run(
        ["qlmanage", "-t", "-s", "1000", "-o", str(sortie), *fichiers],
        capture_output=True,
        check=False,
    )
    for image in sorted(sortie.glob("*.png")):
        print(f"  {image}")


if __name__ == "__main__":
    main()
