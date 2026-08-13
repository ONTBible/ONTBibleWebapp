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

QuickLook ne rend pas les masques CSS pointant un SVG externe : le signe de
section apparaît donc comme un blanc entre deux filets. Ce n'est pas un défaut
de la page — un vrai navigateur l'affiche. Ne pas « corriger » ce vide.
"""

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
    (RACINE / "pkg/apercu.css").write_text(
        css.replace('url("/fontes/', 'url("../fontes/').replace('url("/images/', 'url("../images/')
    )

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
