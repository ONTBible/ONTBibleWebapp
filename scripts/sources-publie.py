#!/usr/bin/env python3
"""Publie `dist/sources/` pour que les fiches hébraïques arrivent sans build.

## Ce que ça débloque

Une fiche du lexique voyage déjà par le réseau : sa définition vit dans
`glossary.json`, que `CorpusUpdater` va chercher. Mais **le mot hébreu
touchable, non** — la jointure par numéro de Strong est faite par le pipeline à
l'émission, et son résultat vit dans `sources/*.json`, un dossier que
`CorpusUpdater.Manifest.tout` ne nomme pas. L'app le lisait donc depuis son
**bundle**.

Conséquence : rendre un mot hébreu touchable exigeait une soumission à l'App
Store. Quarante-six fiches écrites dans la nuit du 11 septembre 2026 n'auraient
pas atteint la version alors en revue — elle avait été compilée avant elles.

Décidé par l'auteur le 12 septembre : « il faut aussi que les fiches hébreux
puissent être ajoutées même sans build. » `SourcesUpdater` sait télécharger ;
il n'y avait rien à télécharger. C'est ce script.

## Le contrat, lu dans le client et non résumé de mémoire

`SourcesUpdater.swift` sur `origin/device` :

    l.236   GET <origine>/sources/manifeste.json
    l.246   GET <origine>/<fichier.chemin>
    l.293   écrit sur disque au **même** chemin que le manifeste donne

`origine` vaut `https://ontbible.com/`, et les `chemin` du manifeste commencent
déjà par `sources/`. La répétition est voulue, et son commentaire dit pourquoi :
**« la même chaîne désigne le fichier dans le paquet, sur le disque et chez le
publieur. »**

### Le manifeste est publié **verbatim**, aux octets

Pas seulement « sans retoucher les champs » : **sans le re-sérialiser**. Le
client le décode avec `ONTSources.Manifeste`, le même décodeur que le bundle, et
l'écrit dans les octets reçus. Un `json.dumps` d'un dictionnaire relu
changerait l'ordre des clés ou l'échappement sans changer le sens — et le sens
n'est pas ce qu'on compare quand on compare des octets.

**Trois conséquences qu'on ne devine pas :**

- **on ne réécrit pas les `chemin`.** C'était le premier choix de ce publieur, et
  il était faux : un nom par contenu ferait diverger la mise en page du disque de
  celle du paquet, et le même lecteur ne pourrait plus lire l'un ou l'autre
  indifféremment. C'est le client qui a tranché, en le disant dans son en-tête ;
- **donc pas de cache immuable.** Les noms sont fixes, et un cache long devient
  un *risque de corrélation* : un ancien fichier servi sous un manifeste neuf
  fait échouer l'empreinte, jette le candidat entier, et `synchroniser()` rend
  `0` — la même valeur que « rien n'a changé ». Le gel serait muet. Cinq minutes
  et une invalidation à chaque publication bornent la fenêtre ;
- **on ne filtre pas.** Le bundle ne porte qu'un témoin sur cinq, et la tentation
  est de ne publier que celui-là. Impossible : filtrer demanderait de retoucher
  le manifeste. Si un témoin ne doit pas partir, c'est au pipeline de ne pas
  l'émettre — un filtre ici serait une seconde décision sur ce que le corpus
  contient, prise à l'endroit qui en sait le moins.

### Les empreintes sont **pleines**, et on ne les recalcule que pour comparer

Soixante-quatre hexadécimaux, posées par le pipeline. **Pas** la forme tronquée
à douze signes de `corpus-publie.py` : là-bas, l'empreinte est un *détecteur de
changement* qui entre dans le nom du fichier ; ici, c'est une *vérification
d'intégrité* sur les octets reçus. Tronquer le premier est sans conséquence,
tronquer le second en a une.

On les recalcule quand même — pour **comparer**, jamais pour republier. Voir
`verifier_les_empreintes`.
"""

import hashlib
import importlib.util
import json
import pathlib
import re
import shutil
import sys

RACINE = pathlib.Path(__file__).resolve().parent.parent
SOURCE = RACINE / ".." / "ONTBibleApp" / "dist" / "sources"
SORTIE = RACINE / "target" / "sources"

# Le schéma que la liseuse accepte, **en égalité stricte** : `guard
# manifeste.schema == Self.schema` dans `SourcesUpdater.swift`, sinon elle jette
# et reste sur son bundle.
#
# Il est écrit ici et non relevé chez la liseuse, faute de pouvoir : elle vit sur
# `device`, et la CI du site clone `dev`. `verifier_le_schema` le relève quand
# elle est là et se contente de ce littéral sinon — en le disant.
SCHEMA_ATTENDU = 1

LISEUSE = "app/Packages/ONTData/Sources/ONTData/Remote/SourcesUpdater.swift"
MOTIF_DU_SCHEMA = re.compile(r"static\s+let\s+schema\s*=\s*(\d+)")

REFUS = 2


class Refus(Exception):
    """Un refus délibéré de publier — pas une panne."""


def _regle_de_la_date():
    """Rend `verifier_la_date` de `corpus-publie.py`, sans la recopier.

    La règle est la même des deux côtés — c'est **la même estampille**, celle du
    dernier commit du vault, et le client le dit : « la même valeur au caractère
    près que le `generatedAt` du corpus sorti du même passage ».

    Une seconde écriture de la règle finirait par accepter ici ce qu'on refuse
    là, et le désaccord porterait précisément sur le champ qui sert à ordonner
    deux générations. Le trait d'union du nom de fichier interdit un `import`
    ordinaire ; `importlib` le contourne sans dupliquer quoi que ce soit.
    """
    chemin = RACINE / "scripts" / "corpus-publie.py"
    spec = importlib.util.spec_from_file_location("corpus_publie", chemin)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module.verifier_la_date, module.Refus


def verifier_le_schema(manifeste: dict) -> None:
    """Refuse un schéma que la liseuse n'accepte pas.

    La comparaison est en égalité stricte de son côté : un manifeste d'un autre
    schéma est refusé **en bloc**, et elle reste sur son bundle. Mieux vaut ne
    rien publier que publier ce que personne ne lira.

    On relève la constante **chez la liseuse** quand elle est accessible — c'est
    la seule mesure qui ne puisse pas se périmer. Quand elle ne l'est pas, on le
    **dit** : un littéral silencieux se relit comme une mesure.
    """
    schema = manifeste.get("schema")
    if not isinstance(schema, int) or isinstance(schema, bool):
        raise Refus(
            f"  sources/manifeste.json porte « schema » = {schema!r}.\n"
            "  La liseuse le compare à un entier ; tout le reste se lit comme un\n"
            "  schéma inconnu, et la génération est refusée en bloc."
        )

    fichier = SOURCE.parent.parent / LISEUSE
    if fichier.exists():
        trouve = MOTIF_DU_SCHEMA.search(fichier.read_text())
        if trouve is None:
            raise Refus(
                f"  La constante `static let schema` est introuvable dans\n"
                f"  {LISEUSE}. Cette garde ne sait plus ce qu'elle garde, donc\n"
                "  elle ne laisse pas passer — si la constante a changé de forme,\n"
                "  c'est MOTIF_DU_SCHEMA qu'il faut remettre d'accord avec elle."
            )
        attendu = int(trouve.group(1))
        origine = "relevé chez la liseuse"
    else:
        attendu = SCHEMA_ATTENDU
        origine = "littéral — la liseuse n'est pas sur la branche clonée"
        print(
            f"  ⚠ {LISEUSE} introuvable : le schéma attendu vaut {attendu}\n"
            "    par défaut et non par mesure. Il sera relevé quand la liseuse\n"
            "    atteindra la branche que la CI clone."
        )

    if schema != attendu:
        raise Refus(
            f"  Le manifeste porte le schéma {schema}, la liseuse lit le {attendu}\n"
            f"  ({origine}).\n"
            "  La comparaison est en égalité stricte : elle refuserait la\n"
            "  génération entière et garderait son bundle.\n"
            "  L'ordre est : livrer la liseuse qui sait lire, puis publier."
        )
    print(f"  schéma {schema} — {origine}")


def fichiers_annonces(manifeste: dict) -> list[dict]:
    """Tout ce que le manifeste promet, à plat.

    Un livre dont les `temoins` sont vides est **complet par déclaration** : son
    absence est du contrat, pas un trou. C'est le cas de six des sept livres
    aujourd'hui, et les compter comme manquants refuserait toute publication.
    """
    annonces = []
    for livre, contenu in (manifeste.get("livres") or {}).items():
        for temoin, fichier in (contenu.get("temoins") or {}).items():
            annonces.append({"livre": livre, "temoin": temoin, **fichier})
    return annonces


def verifier_les_empreintes(annonces: list[dict]) -> None:
    """Refuse si un fichier manque, ou si ses octets ne sont pas ceux annoncés.

    ## Pourquoi vérifier ce que le pipeline a déjà calculé

    Parce que la liseuse le vérifiera, et **plus tard** : elle prouve chaque
    fichier sur les octets reçus, et un seul échec jette la génération entière.
    Son refus rend `0`, qui vaut aussi « rien n'a changé » — donc son échec est
    **muet**.

    La même mesure, faite ici, échoue **avant** la publication, chez quelqu'un
    qui regarde. Ce n'est pas une seconde normalisation : le résultat n'est écrit
    nulle part, il ne sert qu'à rougir.

    `octets` est vérifié aussi, bien que l'empreinte le couvre : le message dit
    alors « tronqué » plutôt que « différent », et les deux ne mènent pas
    au même endroit.
    """
    manquants, faux = [], []
    for a in annonces:
        fichier = SOURCE.parent / a["chemin"]
        if not fichier.exists():
            manquants.append(a["chemin"])
            continue
        octets = fichier.read_bytes()
        if len(octets) != a.get("octets"):
            faux.append(
                f"{a['chemin']} : {len(octets)} octets,"
                f" {a.get('octets')} annoncés"
            )
            continue
        mesuree = hashlib.sha256(octets).hexdigest()
        if mesuree != a.get("sha256"):
            faux.append(
                f"{a['chemin']} : empreinte {mesuree[:12]}…,"
                f" {str(a.get('sha256'))[:12]}… annoncée"
            )

    if manquants:
        raise Refus(
            "  Le manifeste annonce des fichiers qui n'existent pas :\n"
            + "".join(f"    {c}\n" for c in manquants)
            + "  La liseuse jetterait la génération entière, sans le dire."
        )
    if faux:
        raise Refus(
            "  Des fichiers ne sont pas ceux que le manifeste annonce :\n"
            + "".join(f"    {f}\n" for f in faux)
            + "  Publier les octets sous une empreinte fausse ferait échouer la\n"
            "  liseuse après téléchargement, et son échec est muet."
        )


def main() -> None:
    if not SOURCE.exists():
        raise SystemExit(
            f"{SOURCE} introuvable — le pipeline de ONTBibleApp doit avoir tourné"
        )

    # ── Avant d'écrire quoi que ce soit ─────────────────────────────────────
    #
    # Les octets du manifeste sont lus **une fois** et republiés tels quels. On
    # analyse une copie pour les gardes ; c'est l'original qui part.
    octets_du_manifeste = (SOURCE / "manifeste.json").read_bytes()
    manifeste = json.loads(octets_du_manifeste)

    verifier_le_schema(manifeste)

    # La **règle** est celle du corpus, partagée à dessein. Son **message** ne
    # l'est pas : elle parle de `dist/manifest.json` et de `generatedAt`, qui
    # sont les noms de là-bas. Ici le fichier est `sources/manifeste.json` et le
    # champ s'appelle `genere`.
    #
    # Laissé tel quel, le refus enverrait chercher un champ qui existe, dans un
    # fichier qui n'est pas celui en cause — un diagnostic bien formé qui désigne
    # le mauvais objet. On garde donc la règle et on renomme la plainte.
    verifier_la_date, refus_du_corpus = _regle_de_la_date()
    try:
        verifier_la_date(manifeste.get("genere", ""))
    except refus_du_corpus as cause:
        plainte = (
            str(cause)
            .replace("dist/manifest.json", "sources/manifeste.json")
            .replace("generatedAt", "genere")
            .replace(
                "C'est `generated_at` dans le pipeline qu'il faut remplir",
                "C'est `ONT_GENERE` qu'il faut poser dans l'environnement du"
                " pipeline",
            )
        )
        raise Refus(plainte) from cause

    annonces = fichiers_annonces(manifeste)

    # Le témoin positif. Un manifeste qui n'annonce **rien** passerait toutes les
    # gardes et publierait un dossier vide — que la liseuse accepterait comme
    # une génération complète, puisque « un livre à témoins vide est complet par
    # déclaration ». Elle effacerait alors ce qu'elle avait.
    #
    # Ne rien trouver n'est pas trouver zéro : c'est le relevé qui est cassé.
    if not annonces:
        raise Refus(
            "  Le manifeste n'annonce aucun fichier.\n"
            "  Publié tel quel, il décrirait une génération vide et complète —\n"
            "  la liseuse s'y fierait et perdrait ce qu'elle porte.\n"
            "  Le pipeline a dû changer la forme de `livres`."
        )

    verifier_les_empreintes(annonces)

    # ── Maintenant on écrit ─────────────────────────────────────────────────
    if SORTIE.exists():
        shutil.rmtree(SORTIE)
    SORTIE.mkdir(parents=True)

    total = 0
    for a in annonces:
        # Le chemin du manifeste est **la** convention : il désigne le fichier
        # dans le paquet, sur le disque du lecteur, et ici. On le suit sans le
        # transformer, en retirant le seul segment `sources/` qui le préfixe —
        # SORTIE *est* ce dossier.
        relatif = a["chemin"].removeprefix("sources/")
        destination = SORTIE / relatif
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(SOURCE.parent / a["chemin"], destination)
        total += a["octets"]

    # Le manifeste **en dernier**, et dans ses octets d'origine. Annoncer un
    # fichier qui n'est pas encore en ligne fait rater un tour à la liseuse,
    # sans casse ; l'inverse n'est pas vrai.
    (SORTIE / "manifeste.json").write_bytes(octets_du_manifeste)

    temoins = {a["temoin"] for a in annonces}
    print(
        f"  {len(annonces)} fichier(s), {len(temoins)} témoin(s),"
        f" {total / 1024:.0f} Ko"
    )
    print(f"  manifeste : {len(octets_du_manifeste)} octets, republiés tels quels")
    print(f"  → {SORTIE}")


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Refus as refus:
        print(f"\nPublication refusée :\n{refus}", file=sys.stderr)
        sys.exit(REFUS)
