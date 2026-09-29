#!/usr/bin/env python3

"""Met une capture de l'app et un rendu du site côte à côte.

## Pourquoi un outil et pas des commandes

L'auteur a demandé le 29 septembre 2026 « un rendu UI comme si c'était un
fork », et la méthode avec :

> hésite pas à passer ton temps à faire des comparaisons des screens entre
> l'app iOS et le rendu responsive, pareil pour le rendu desktop et l'app macOS

Une comparaison se refait **à chaque correction**. Faite à la main, elle coûte
cinq commandes, et l'on finit par la sauter — donc par corriger à l'aveugle.
Faite par un script, elle coûte une ligne, et c'est elle qui dit si la
correction a marché.

C'est la même raison que `scripts/portes.sh` du 16 août, qui comparait quatre
dessins de seuil : **un choix visuel demande un banc, et le banc se jette
quand le choix est fait.** Celui-ci ne se jettera pas — il ne sert pas à
choisir une fois, il sert à mesurer un écart qui doit tendre vers zéro.

## Ce qu'il ne fait pas

Il ne dit pas si c'est *juste*. Il met deux images côte à côte et l'œil
tranche. C'est délibéré : aucun test ne sait dire qu'une liste « fait app »,
et prétendre le contraire produirait une garde qui rassure sans regarder.

## Les deux axes, et ils n'ont pas la même référence

    iphone   app/Captures/brut/iphone-6.9   ↔  le site au simulateur
    mac      app/Captures/mac               ↔  le site à 1440 × 900

Le premier passe par `sim.sh`, parce que c'est la seule mesure qui vaille pour
ce qui dépend de la largeur (§7 bis). Le second passe par QuickLook, qui rend
à une fenêtre large.

## Emploi

    ./scripts/comparer-a-l-app.py iphone 01=/fr/lire 04=/fr/lire/bereshit
    ./scripts/comparer-a-l-app.py mac    01=/fr/lire

La clé à gauche du `=` est le numéro de la capture de l'app ; à droite,
l'adresse du site à mettre en face.
"""

import pathlib
import subprocess
import sys

RACINE = pathlib.Path(__file__).resolve().parent.parent
APP = RACINE.parent / "ONTBibleApp" / "app" / "Captures"
SERVEUR = "http://127.0.0.1:3000"

AXES = {
    "iphone": APP / "brut" / "iphone-6.9",
    "ipad": APP / "brut" / "ipad-13",
    "mac": APP / "mac",
}


class Refus(Exception):
    """Un refus délibéré — pas une panne."""


def feuille_nue() -> None:
    """Dépose `ontbible.css` à côté de sa copie empreintée.

    `apercu.py` refuse si les deux diffèrent — c'est son témoin de péremption,
    et il a raison : `cargo leptos watch` régénère la fraîche sans l'empreintée,
    et l'aperçu montrerait alors le style du dernier redémarrage complet.

    Mais après un `cargo leptos serve`, c'est l'inverse : seule l'empreintée
    existe, et le témoin refuse faute de pouvoir comparer. On dépose donc la
    copie — dans **ce sens-là**, elle ne peut pas mentir, puisqu'elle est faite
    depuis le fichier que la page référence.
    """
    pkg = RACINE / "target" / "site" / "pkg"
    empreintees = [f for f in pkg.glob("ontbible.*.css")]
    if len(empreintees) == 1:
        (pkg / "ontbible.css").write_bytes(empreintees[0].read_bytes())


def sans_ouverture(chemin: str) -> str:
    """Sert la page avec l'ouverture déjà vue, et rend son adresse locale.

    L'ouverture couvre l'écran cinq secondes et demie, et aucun de nos deux
    outils de capture ne sait attendre. Une comparaison qui la photographie
    compare une animation à une liste.

    On pose donc la classe que le script de l'en-tête poserait si la session
    l'avait déjà vue — c'est-à-dire l'état de tout lecteur sauf au premier
    instant de sa première visite. La page est déposée dans `target/site/`, que
    le serveur de développement sert à la racine.
    """
    import re
    import urllib.request

    document = urllib.request.urlopen(SERVEUR + chemin).read().decode()
    document = document.replace('<html lang="fr">', '<html lang="fr" class="deja-entre">', 1)
    # Le nom ne porte que des lettres et des tirets : un `%2F` échappé s'y
    # redécoderait en barre à la requête suivante, et le serveur chercherait un
    # dossier qui n'existe pas. Mesuré — un 404 sans rapport avec la page.
    nom = "banc-" + re.sub(r"[^a-z0-9]+", "-", chemin.strip("/").lower()).strip("-") + ".html"
    (RACINE / "target" / "site" / nom).write_text(document)
    return "/" + nom


def rendre_au_simulateur(chemin: str, sortie: pathlib.Path) -> None:
    """Le site sur un vrai Safari, à la largeur d'un téléphone."""
    rendu = subprocess.run(
        ["./scripts/sim.sh", sans_ouverture(chemin), str(sortie)],
        cwd=RACINE,
        capture_output=True,
        text=True,
        env={**__import__("os").environ, "ATTENTE": "16"},
    )
    if rendu.returncode != 0:
        raise Refus(
            f"  le simulateur a refusé « {chemin} » :\n"
            + "".join(f"    {l}\n" for l in rendu.stderr.splitlines()[-4:])
        )


def rendre_au_large(chemin: str, sortie: pathlib.Path) -> None:
    """Le site sur grand écran, par QuickLook."""
    dossier = sortie.parent / "large"
    dossier.mkdir(parents=True, exist_ok=True)
    rendu = subprocess.run(
        ["./scripts/apercu.py", str(dossier), f"vue={sans_ouverture(chemin)}"],
        cwd=RACINE,
        capture_output=True,
        text=True,
    )
    if rendu.returncode != 0:
        raise Refus(
            "  `apercu.py` a refusé — le serveur tourne-t-il ?\n"
            + "".join(f"    {l}\n" for l in rendu.stdout.splitlines()[-3:])
        )
    produite = dossier / "apercu-vue.html.png"
    if not produite.exists():
        raise Refus(f"  rien n'est sorti pour « {chemin} »")
    produite.replace(sortie)


def composer(paires: list[tuple[pathlib.Path, pathlib.Path, str]], sortie: pathlib.Path) -> None:
    from PIL import Image, ImageDraw, ImageFont

    HAUTEUR, MARGE, LEGENDE = 1150, 18, 44
    colonnes = []
    for capture, rendu, titre in paires:
        for source, qui in ((capture, "APP"), (rendu, "SITE")):
            image = Image.open(source).convert("RGB")
            largeur = int(image.width * HAUTEUR / image.height)
            colonnes.append((image.resize((largeur, HAUTEUR)), f"{titre} · {qui}"))

    largeur = sum(c[0].width for c in colonnes) + MARGE * (len(colonnes) + 1)
    planche = Image.new("RGB", (largeur, HAUTEUR + LEGENDE + MARGE), "#18090D")
    dessin = ImageDraw.Draw(planche)
    try:
        police = ImageFont.truetype("/System/Library/Fonts/Supplemental/Futura.ttc", 26)
    except OSError:
        police = ImageFont.load_default()

    x = MARGE
    for image, legende in colonnes:
        dessin.text((x, 10), legende, font=police, fill="#CDBE83" if "APP" in legende else "#D87994")
        planche.paste(image, (x, LEGENDE))
        x += image.width + MARGE
    planche.resize((planche.width // 2, planche.height // 2)).save(sortie)


def main() -> None:
    if len(sys.argv) < 3 or sys.argv[1] not in AXES:
        raise SystemExit(__doc__)

    axe = sys.argv[1]
    dossier = AXES[axe]
    if not dossier.is_dir():
        raise Refus(
            f"  {dossier} n'existe pas.\n"
            "  Les captures brutes sont régénérables et non commitées :\n"
            "  `ONTBibleApp/scripts/captures.sh` les refait."
        )

    travail = pathlib.Path("/tmp/ont-comparaisons")
    travail.mkdir(parents=True, exist_ok=True)
    feuille_nue()

    paires = []
    for argument in sys.argv[2:]:
        numero, chemin = argument.split("=", 1)
        capture = dossier / f"{numero}.png"
        if not capture.exists():
            raise Refus(f"  la capture {capture} n'existe pas")
        rendu = travail / f"{axe}-{numero}.png"
        if axe == "iphone":
            rendre_au_simulateur(chemin, rendu)
        else:
            rendre_au_large(chemin, rendu)
        paires.append((capture, rendu, chemin))
        print(f"  {numero}.png  ↔  {chemin}")

    sortie = travail / f"comparaison-{axe}.png"
    composer(paires, sortie)
    print(f"→ {sortie}")


if __name__ == "__main__":
    try:
        main()
    except Refus as refus:
        print(f"\nComparaison refusée :\n{refus}", file=sys.stderr)
        sys.exit(2)
