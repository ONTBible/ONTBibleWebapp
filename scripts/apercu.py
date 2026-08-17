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

## La feuille se lit dans la page, elle ne se devine pas

Corrigé le 15 août 2026. Le script prenait la **première** feuille du dossier,
par ordre alphabétique. Tant qu'il n'y en a qu'une, ça marche.

Il y en a deux dès qu'on a lancé `dev-sync-empreintes.sh` : `ontbible.css` et
sa copie empreintée `ontbible.<empreinte>.css`. L'ordre alphabétique désigne la
première, la page référence la seconde — et le remplacement ne trouvait alors
rien à remplacer. L'aperçu chargeait donc la **vraie** feuille, dont les masques
pointent `/images/…` en absolu, chemins qu'un fichier ouvert depuis le disque ne
résout pas.

Résultat : les masques revenaient en blancs, exactement le défaut que ce script
existe pour éviter. On croit alors que la montagne a disparu de la page, on va
la chercher dans le code, et elle n'a jamais bougé. Une heure perdue le jour où
le QR en a reçu une.

La feuille se lit donc dans le HTML servi. Et si sa copie empreintée diffère de
`ontbible.css`, le script le dit au lieu de rendre une page périmée — c'est le
piège du §7 bis, et il ne se voit pas autrement.
"""

import base64
import pathlib
import re
import subprocess
import sys
import urllib.request

SERVEUR = "http://127.0.0.1:3000"
RACINE = pathlib.Path(__file__).resolve().parent.parent / "target" / "site"


def feuille_de(html: str) -> pathlib.Path:
    """La feuille de style **que la page référence**, et rien d'autre.

    Elle porte une empreinte dans son nom — `ontbible.<empreinte>.css`, voir le
    §8 ter — qui change à chaque retouche du style. La deviner par un tri du
    dossier marchait tant qu'il n'y avait qu'un fichier ; il y en a deux dès
    qu'on a lancé `dev-sync-empreintes.sh`.
    """
    trouve = re.search(r"pkg/(ontbible[\w.-]*\.css)", html)
    if not trouve:
        raise SystemExit(
            "La page ne référence aucune feuille. Le serveur tourne-t-il ?\n"
            "`cargo leptos watch`, puis attendre la fin de la construction."
        )

    feuille = RACINE / "pkg" / trouve.group(1)
    if not feuille.exists():
        raise SystemExit(
            f"La page demande {feuille.name}, absente de target/site/pkg/.\n"
            "C'est le §7 bis : lancez `./scripts/dev-sync-empreintes.sh`."
        )

    # Le témoin de péremption. `cargo leptos watch` régénère `ontbible.css` sans
    # refaire la copie empreintée ; la page sert alors le style du dernier
    # redémarrage complet, et l'aperçu montrerait une correction qui n'y est
    # pas. C'est une heure déjà perdue, dans une discussion sur un rendu qui
    # n'était pas celui du code.
    fraiche = RACINE / "pkg" / "ontbible.css"
    if fraiche.exists() and fraiche.read_bytes() != feuille.read_bytes():
        raise SystemExit(
            f"{feuille.name} est périmée — `ontbible.css` a changé depuis.\n"
            "Lancez `./scripts/dev-sync-empreintes.sh`, puis recommencez."
        )

    return feuille


def main() -> None:
    if len(sys.argv) < 3:
        raise SystemExit(__doc__)

    sortie = pathlib.Path(sys.argv[1])
    sortie.mkdir(parents=True, exist_ok=True)

    # Les pages d'abord : c'est **elles** qui nomment la feuille à charger.
    pages = []
    for argument in sys.argv[2:]:
        nom, chemin = argument.split("=", 1)
        pages.append((nom, urllib.request.urlopen(f"{SERVEUR}{chemin}").read().decode()))

    feuille = feuille_de(pages[0][1])

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
    for nom, html in pages:
        html = re.sub(r'(src|srcset|href)="/', r'\1="', html)
        avant = html
        html = html.replace(f"pkg/{feuille.name}", "pkg/apercu.css")
        if html == avant:
            # Le défaut de l'ancienne version, rendu bruyant : sans ce
            # remplacement la page charge la vraie feuille, et tous les masques
            # arrivent en blancs sans que rien ne le signale.
            raise SystemExit(f"« {nom} » ne référence pas {feuille.name} — aperçu abandonné")
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
