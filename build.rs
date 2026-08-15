//! Ce que le site sait à la compilation.
//!
//! Deux choses y sont figées, et pour la même raison : elles ne changent qu'au
//! déploiement, et les lire à l'exécution coûterait un accès au disque à
//! chaque requête pour un résultat identique.
//!
//! ## L'année, pour la mention de droit d'auteur
//!
//! Le pied de page est rendu par le serveur **et** par le navigateur : il ne
//! peut donc pas lire l'horloge, qui n'existe que d'un côté. Et une année
//! écrite en dur se périme sans que personne ne s'en aperçoive.
//!
//! ## L'état du corpus
//!
//! Les chiffres viennent du `manifest.json` que produit le pipeline de
//! `ONTBibleApp` — **jamais recopiés**. Un site qui annonce « trois livres »
//! alors que le vault en a cinq ment sans que personne ne le remarque, parce
//! que rien ne compare les deux.
//!
//! Si le dépôt voisin manque, la compilation échoue avec le chemin en clair.
//! C'est voulu : mieux vaut un build qui refuse qu'un site qui invente.

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("cargo:rustc-env=ANNEE_DE_COMPILATION={}", annee());
    etat_du_corpus();
    livres_du_corpus();
    empreintes_des_images();
}

/// L'empreinte de chaque image, en code.
///
/// ## Le défaut que ça corrige
///
/// Les feuilles et le WASM portent une empreinte dans leur **nom** :
/// `ontbible.<empreinte>.css`. Un contenu nouveau a donc une adresse nouvelle,
/// et ne peut pas être servi depuis un cache. Voir le §8 ter.
///
/// Les images n'ont jamais eu ce traitement. `wordmark.svg` reste
/// `wordmark.svg` quand son dessin change, et il est servi avec un cache d'une
/// journée : **une image corrigée reste invisible jusqu'à vingt-quatre heures**
/// pour qui a déjà visité le site.
///
/// Ce n'est pas théorique. Le ® a été retiré de la marque le 13 août 2026, et
/// le lendemain on en débattait encore en regardant deux versions différentes
/// du même fichier — l'une servie par la production, l'autre par un cache de
/// navigateur.
///
/// ## Pourquoi un paramètre et non un nom
///
/// Renommer les fichiers demanderait de les recopier au build, et `public/` est
/// posé tel quel sur le seau. Le paramètre `?v=` change l'adresse sans toucher
/// au fichier : le navigateur y voit une autre ressource, S3 l'ignore, et
/// l'invalidation CloudFront du déploiement s'occupe du bord.
///
/// ## Pourquoi une empreinte maison
///
/// Ce qu'on demande à cette valeur, c'est de **changer quand le contenu
/// change** — rien de plus. Une collision ne serait pas une faille, seulement
/// une image périmée de plus, ce qu'on a déjà aujourd'hui sur toutes. FNV-1a
/// tient en huit lignes et n'ajoute aucune dépendance à un site qui n'en a que
/// ce qu'il lui faut.
/// ## Le `rerun-if-changed` porte sur **chaque fichier**, pas seulement sur le
/// dossier
///
/// Corrigé le 15 août 2026, et le défaut annulait tout ce qui précède.
///
/// La première version ne déclarait que le dossier. Cargo en regarde alors la
/// date de modification — qui bouge quand une image apparaît ou disparaît, et
/// **pas** quand on en réécrit une existante. Or c'est exactement le cas qu'on
/// veut couvrir : une image *corrigée*, sous le même nom.
///
/// La table gardait donc l'ancienne empreinte, l'adresse ne changeait pas, et
/// le navigateur resservait son cache. Le mécanisme entier — un `?v=` par
/// contenu, une invalidation CloudFront au déploiement — ne protégeait que du
/// cas qui n'arrive jamais.
///
/// Trouvé en régénérant le QR : le fichier changeait sur le disque, la page
/// continuait d'afficher `?v=2m516j8kqleko`. Le même piège qu'au §8 quater bis,
/// rejoué un cran plus bas.
///
/// `livres_du_corpus` n'en souffre pas, et son commentaire dit pourquoi : ses
/// fichiers sont suivis par `rustc` du fait de l'`include_str!`. Ici rien ne les
/// suit — on les lit, on les résume, et on jette. Il faut donc le dire.
fn empreintes_des_images() {
    let dossier = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("public/images");

    // Le dossier pour les apparitions et les disparitions.
    println!("cargo:rerun-if-changed={}", dossier.display());

    let mut entrees: Vec<(String, String)> = fs::read_dir(&dossier)
        .expect("public/images illisible")
        .filter_map(Result::ok)
        .map(|entree| entree.path())
        .filter(|chemin| chemin.is_file())
        .filter_map(|chemin| {
            // Et chaque fichier pour les corrections, qui ne touchent pas à la
            // date du dossier.
            println!("cargo:rerun-if-changed={}", chemin.display());
            let nom = chemin.file_name()?.to_str()?.to_string();
            let octets = fs::read(&chemin).ok()?;
            Some((nom, empreinte(&octets)))
        })
        .collect();

    // Trié : un tableau dont l'ordre change à chaque build ferait recompiler
    // tout ce qui en dépend sans qu'une seule image ait bougé.
    entrees.sort();

    let mut code = String::from("pub static EMPREINTES: &[(&str, &str)] = &[\n");
    for (nom, empreinte) in &entrees {
        code.push_str(&format!("    ({nom:?}, {empreinte:?}),\n"));
    }
    code.push_str("];\n");

    fs::write(
        PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR")).join("images.rs"),
        code,
    )
    .expect("écriture de images.rs");
}

/// FNV-1a sur 64 bits, rendue en base 36.
fn empreinte(octets: &[u8]) -> String {
    let mut somme: u64 = 0xcbf2_9ce4_8422_2325;
    for octet in octets {
        somme ^= *octet as u64;
        somme = somme.wrapping_mul(0x1000_0000_01b3);
    }

    let alphabet = b"0123456789abcdefghijklmnopqrstuvwxyz";
    let mut sortie = Vec::new();
    let mut reste = somme;
    while reste > 0 {
        sortie.push(alphabet[(reste % 36) as usize]);
        reste /= 36;
    }
    sortie.reverse();
    String::from_utf8(sortie).expect("base 36 est de l'ASCII")
}

/// La liste des livres écrits, en code.
///
/// Le site embarque le corpus à la compilation — c'est ce qui lui évite de
/// déployer un dossier de données à côté du binaire, et de tomber le jour où
/// ce dossier manque. Mais `include_str!` veut un chemin **littéral** : on ne
/// peut pas écrire une boucle qui en embarque une liste variable.
///
/// D'où ce générateur. Il lit `dist/books/`, et écrit un fichier de code qui
/// contient un `include_str!` par livre. Le jour où le vault en compte cinq, le
/// tableau en compte cinq — sans que personne n'ait à y penser.
///
/// Le `rerun-if-changed` porte sur le **dossier** : c'est ce qui fait
/// recompiler quand un livre apparaît ou disparaît. Le contenu de chaque
/// fichier, lui, est déjà suivi par `rustc` du fait de l'`include_str!`.
fn livres_du_corpus() {
    let dossier = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../ONTBibleApp/dist/books")
        .canonicalize()
        .expect("dist/books introuvable — le dépôt ONTBibleApp doit être à côté");

    println!("cargo:rerun-if-changed={}", dossier.display());

    let mut livres: Vec<(String, PathBuf)> = fs::read_dir(&dossier)
        .expect("dist/books illisible")
        .filter_map(Result::ok)
        .map(|entree| entree.path())
        .filter(|chemin| chemin.extension().is_some_and(|e| e == "json"))
        .map(|chemin| {
            let id = chemin
                .file_stem()
                .expect("un fichier .json a un nom")
                .to_string_lossy()
                .into_owned();
            (id, chemin)
        })
        .collect();

    // L'ordre du système de fichiers n'est pas garanti d'une machine à
    // l'autre. Sans ce tri, deux compilations du même code produiraient deux
    // binaires différents — et un diff de build illisible.
    livres.sort();

    let entrees: String = livres
        .iter()
        .map(|(id, chemin)| format!("    (\"{id}\", include_str!(r\"{}\")),\n", chemin.display()))
        .collect();

    let code = format!(
        "/// Les livres écrits, embarqués à la compilation. Généré par `build.rs`.\n\
         pub static LIVRES: &[(&str, &str)] = &[\n{entrees}];\n"
    );

    let sortie = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR")).join("livres.rs");
    fs::write(&sortie, code).expect("écriture de livres.rs");
}

fn etat_du_corpus() {
    let manifeste = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../ONTBibleApp/dist/manifest.json")
        .canonicalize()
        .expect("dist/manifest.json introuvable — le dépôt ONTBibleApp doit être à côté");

    println!("cargo:rerun-if-changed={}", manifeste.display());
    let source = fs::read_to_string(&manifeste).expect("manifest.json illisible");

    // Une lecture au motif plutôt qu'un analyseur JSON : `build.rs` n'a pas
    // accès aux dépendances du paquet, et faire entrer `serde_json` en
    // dépendance de compilation pour cinq entiers serait cher payé.
    for (cle, variable) in [
        ("books", "CORPUS_LIVRES"),
        ("booksWritten", "CORPUS_LIVRES_ECRITS"),
        ("chapters", "CORPUS_UNITES"),
        ("verses", "CORPUS_VERSETS"),
        ("glossaryEntries", "CORPUS_LEXIQUE"),
    ] {
        let valeur = entier(&source, cle)
            .unwrap_or_else(|| panic!("clé « {cle} » absente de manifest.json"));
        println!("cargo:rustc-env={variable}={valeur}");
    }
}

/// Le premier entier qui suit `"cle":` dans le document.
fn entier(source: &str, cle: &str) -> Option<u32> {
    let debut = source.find(&format!("\"{cle}\""))? + cle.len() + 2;
    let reste = &source[debut..];
    let apres_deux_points = reste.find(':')? + 1;
    reste[apres_deux_points..]
        .trim_start()
        .chars()
        .take_while(char::is_ascii_digit)
        .collect::<String>()
        .parse()
        .ok()
}

/// L'année grégorienne, calculée sans dépendance.
///
/// Le calcul passe par le nombre de jours depuis 1970 et la règle des
/// bissextiles. Une approximation en « 365,25 jours » dériverait d'un jour tous
/// les siècles, et se tromperait donc d'année un 31 décembre.
fn annee() -> i32 {
    let jours = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("horloge antérieure à 1970")
        .as_secs()
        / 86_400;

    let mut annee = 1970;
    let mut restant = jours as i64;
    loop {
        let longueur = if bissextile(annee) { 366 } else { 365 };
        if restant < longueur {
            return annee;
        }
        restant -= longueur;
        annee += 1;
    }
}

fn bissextile(annee: i32) -> bool {
    (annee % 4 == 0 && annee % 100 != 0) || annee % 400 == 0
}
