use leptos::prelude::*;

use crate::interface::design::{Lien, PageLegale};
use crate::interface::pages::confidentialite::CONTACT;
use crate::interface::tete::Tete;

/// `/fr/assistance` — l'adresse d'assistance que l'App Store réclame.
///
/// ## Pourquoi elle existe
///
/// Apple exige une **URL d'assistance** sur toute fiche d'app, et refuse celles
/// qui pointent une page d'accueil ou une adresse morte. Elle doit dire à qui
/// s'adresser et pour quoi.
///
/// Mais elle ne sert pas qu'à ça. Quelqu'un qui lit une traduction en cours et
/// trouve une faute doit savoir où le dire — et pour ce projet-là, c'est une
/// information précieuse, pas une corvée de service après-vente.
///
/// ## Elle ne promet rien qu'on ne tienne
///
/// Pas de délai de réponse, pas de « sous 24 heures ». Le projet est porté par
/// une personne : annoncer une permanence serait une promesse qu'un mois chargé
/// briserait.
#[component]
pub fn Assistance() -> impl IntoView {
    view! {
        <Tete
            titre="Assistance"
            description="Signaler une faute, poser une question, demander l'effacement \
                         de ses données — où écrire et quoi attendre."
            chemin="/fr/assistance"
        />

        <PageLegale titre="Assistance" mise_a_jour="13 août 2026">
            <p>
                "Une seule adresse, pour tout : "
                <Lien href=format!("mailto:{CONTACT}")>{CONTACT}</Lien>"."
            </p>

            <h2>"Signaler une faute dans le texte"</h2>
            <p>
                "C'est ce qui aide le plus. La traduction est en cours, et une paire "
                "d'yeux de plus vaut mieux qu'une relecture de plus."
            </p>
            <p>
                "Indiquez le "<strong>"renvoi"</strong>" — « Bereshit 3:7 » — et ce qui "
                "vous arrête. Une coquille, un mot qui sonne faux, une glose qui dit "
                "autre chose que le verset : tout se prend."
            </p>
            <p>
                "Une correction acceptée atteint l'application "<strong>"en quelques "
                "minutes"</strong>". Le texte n'est pas figé dans la version installée : "
                "l'app va le chercher toute seule."
            </p>

            <h2>"Une unité manque, ou porte « brouillon »"</h2>
            <p>
                "Ce n'est pas une panne. Trois livres sur soixante-dix sont traduits, et "
                "une unité qui n'a pas été relue le dit — c'est délibéré. Le sommaire du "
                <Lien href="/fr/lire">"corpus"</Lien>" montre ce qui se lit aujourd'hui."
            </p>

            <h2>"Un problème technique"</h2>
            <p>
                "Dites ce que vous faisiez, ce que vous attendiez, ce qui est arrivé. "
                "L'appareil et la version d'iOS aident, quand vous les avez sous la main."
            </p>
            <p>
                "L'application signale ses pannes toute seule, sans rien envoyer de ce "
                "que vous lisez ni de ce que vous notez — voir la "
                <Lien href="/fr/confidentialite">"politique de confidentialité"</Lien>"."
            </p>

            <h2>"Vos données"</h2>
            <p>
                "Le compte est facultatif : l'application se lit entièrement sans. Si "
                "vous en avez créé un, "<strong>"Vous → Supprimer mon compte"</strong>
                " efface la copie sur le serveur, sans passer par nous."
            </p>
            <p>
                "Pour toute autre demande — accès, rectification, portabilité — écrivez "
                "à la même adresse."
            </p>

            <h2>"Ce qu'on ne promet pas"</h2>
            <p>
                "Aucun délai de réponse. Ce projet est porté par une personne, et "
                "annoncer une permanence serait une promesse qu'un mois chargé briserait. "
                "Tout ce qui arrive est lu."
            </p>
        </PageLegale>
    }
}
