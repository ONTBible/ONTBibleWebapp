use leptos::prelude::*;

use crate::interface::design::{Lien, PageLegale};
use crate::interface::pages::confidentialite::CONTACT;
use crate::interface::tete::Tete;

/// Les conditions d'utilisation.
#[component]
pub fn Conditions() -> impl IntoView {
    view! {
        <Tete
            titre="Conditions"
            description="Le site et l'application sont fournis tels quels. Le code est \
                         public ; le texte de la traduction reste la propriété de son auteur."
            chemin="/fr/conditions"
        />

        <PageLegale titre="Conditions" mise_a_jour="12 août 2026">
            <p>
                "En utilisant le site "<code>"ontbible.com"</code>" ou l'application "
                "La Bible ONT, vous acceptez ce qui suit."
            </p>

            <h2>"Une traduction en cours"</h2>
            <p>
                "L'ONT n'est pas achevée. Une "<strong>"unité verrouillée"</strong>" a été "
                "relue et validée par son auteur : elle fait référence. Une unité qui ne "
                "l'est pas est un brouillon, et l'app le signale."
            </p>
            <p>
                "Un brouillon peut changer, y compris sur un terme. Ne le citez pas comme un "
                "texte établi."
            </p>

            <h2>"Fournis tels quels"</h2>
            <p>
                "Le site et l'application sont fournis sans garantie d'aucune sorte, expresse "
                "ou implicite. L'auteur n'est pas responsable d'une perte de données ni d'un "
                "dommage découlant de leur usage. Vos annotations vous appartiennent : "
                "gardez-en une copie si elles comptent."
            </p>

            <h2>"Le code est ouvert, le texte ne l'est pas encore"</h2>
            <p>
                "Le code du site, de l'application et du pipeline est public sur "
                <Lien href="https://github.com/ONTBible">"GitHub"</Lien>
                ", avec sa licence."
            </p>
            <p>
                "Le "<strong>"texte de la traduction"</strong>" est autre chose. Il est "
                "consultable publiquement, mais aucune licence ne l'ouvre à la "
                "redistribution ou à la modification : il reste la propriété de son auteur. "
                "Citez-le librement, avec son renvoi ; pour un autre usage, écrivez."
            </p>

            <h2>"Les fontes"</h2>
            <p>
                "Les caractères employés — EB Garamond, Jost, Literata, Frank Ruhl Libre, "
                "Ezra SIL — sont diffusés sous "
                <Lien href="https://openfontlicense.org">"SIL Open Font License"</Lien>
                ". Leur texte de licence accompagne les fichiers, comme cette licence "
                "l'exige."
            </p>

            <h2>"Comptes et services tiers"</h2>
            <p>
                "La connexion passe par Apple, Google ou GitHub. Vous pouvez révoquer "
                "l'autorisation à tout moment depuis les réglages du fournisseur concerné. "
                "La Bible ONT n'est affiliée à aucun d'eux."
            </p>
            <p>
                "Un compte peut être fermé sans préavis en cas d'usage manifestement abusif "
                "— sollicitation automatisée du serveur, tentative d'atteinte au service."
            </p>

            <h2>"Droit applicable"</h2>
            <p>"Droit français."</p>

            <h2>"Contact"</h2>
            <p><Lien href=format!("mailto:{CONTACT}")>{CONTACT}</Lien></p>
        </PageLegale>
    }
}
