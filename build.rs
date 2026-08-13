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
