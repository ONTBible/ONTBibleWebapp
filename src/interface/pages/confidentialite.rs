use leptos::prelude::*;

use crate::interface::design::{Lien, PageLegale};
use crate::interface::tete::Tete;

/// L'adresse de contact.
///
/// Sur le domaine du projet, et non sur une messagerie personnelle : une
/// demande d'effacement doit pouvoir être adressée sans connaître le nom de
/// l'auteur, et l'adresse doit survivre à un changement de fournisseur.
pub const CONTACT: &str = "contact@ontbible.com";

/// La politique de confidentialité.
///
/// Elle décrit ce que l'app fait **réellement** — vérifié dans le code, pas
/// recopié d'un modèle. `ExternalIdentity`, `Highlight` et `Position` sont les
/// trois seules choses qui quittent l'appareil, et `Observability.swift`
/// désactive explicitement les captures d'écran et les données personnelles.
///
/// Une politique fausse est pire qu'une politique absente : elle engage.
#[component]
pub fn Confidentialite() -> impl IntoView {
    view! {
            <Tete
                titre="Confidentialité"
                description="Ce que le site et l'app collectent, et ce qu'ils ne collectent pas. \
                             Aucun traceur sur le site ; sur l'app, le strict nécessaire à la \
                             synchronisation."
                chemin="/fr/confidentialite"
            />

            <PageLegale titre="Confidentialité" mise_a_jour="12 août 2026">
                <p>
                    "Ce document dit ce qui est collecté, pourquoi, où c'est conservé et comment "
                    "le faire effacer. Il vaut pour le site "<code>"ontbible.com"</code>" et pour "
                    "l'application La Bible ONT."
                </p>

                <h2>"Le site ne vous suit pas"</h2>
                <p>
                    "Aucun cookie. Aucun traceur. Aucune mesure d'audience. Aucun service tiers "
                    "n'est appelé depuis vos pages : les fontes et les images sont servies par "
                    "ce domaine, pas par un réseau de diffusion."
                </p>
                <p>
                    "Le serveur écrit des journaux techniques — adresse IP, page demandée, code "
                    "de réponse — que l'hébergeur conserve un temps limité. C'est le minimum "
                    "nécessaire pour qu'un serveur fonctionne et pour repérer un abus."
                </p>

                <h2>"L'application, sans compte"</h2>
                <p>
                    "Sans compte, "<strong>"rien ne quitte votre appareil"</strong>". La lecture, "
                    "les surlignages, les notes et le rappel quotidien fonctionnent hors ligne. "
                    "Le verset du jour est calculé à partir de la date, pas demandé à un serveur."
                </p>

                <h2>"L'application, avec un compte"</h2>
                <p>
                    "Créer un compte sert à retrouver vos annotations sur un autre appareil. "
                    "La connexion passe par Apple, Google ou GitHub. Nous recevons alors :"
                </p>
                <ul>
                    <li>
                        "un "<strong>"identifiant stable"</strong>" chez ce fournisseur — une "
                        "chaîne opaque qui ne dit rien de vous ;"
                    </li>
                    <li>
                        "votre "<strong>"adresse électronique"</strong>", et seulement si le "
                        "fournisseur la transmet. Apple ne la donne qu'à la première "
                        "autorisation, GitHub uniquement si elle est publique."
                    </li>
                </ul>
                <p>
                    "Ni nom, ni photo, ni liste de contacts, ni identifiant publicitaire. "
                    "Le mot de passe de votre fournisseur ne transite jamais par l'app."
                </p>

                <h2>"Ce qui est synchronisé"</h2>
                <ul>
                    <li>"vos surlignages — le passage, la couleur ;"</li>
                    <li>"vos notes ;"</li>
                    <li>"votre position de lecture."</li>
                </ul>
                <p>
                    "Ces données sont conservées en France, sur des serveurs situés dans la "
                    "région "<code>"eu-west-3"</code>" (Paris). Elles ne quittent pas l'Union "
                    "européenne et ne sont ni vendues, ni partagées, ni analysées."
                </p>

                <h2>"Une précaution particulière"</h2>
                <p>
                    "Ce que vous surlignez dans une Bible peut révéler vos convictions "
                    "religieuses. L'article 9 du RGPD range cette information parmi les données "
                    "sensibles, et l'app est conçue en conséquence :"
                </p>
                <ul>
                    <li>
                        "le rappel quotidien est une "<strong>"notification locale"</strong>". "
                        "Il est préparé sur l'appareil à partir de la date : rien n'est demandé "
                        "à un serveur, et il n'existe nulle part de liste de qui lit une Bible "
                        "et à quelle heure ;"
                    </li>
                    <li>
                        "les parutions vous sont signalées "<strong>"sans rien envoyer"</strong>", "
                        "tant que vous n'activez pas les notifications distantes : l'app compare "
                        "le corpus qu'elle vient de télécharger à ce qu'elle en savait, et vous "
                        "prévient elle-même ;"
                    </li>
                    <li>
                        "le rapport d'erreur "<strong>"ne joint ni capture d'écran, ni "
                        "hiérarchie de vues, ni donnée personnelle"</strong>" — ces options sont "
                        "désactivées explicitement dans le code, qui est public ;"
                    </li>
                    <li>"le widget et la lecture fonctionnent en mode avion."</li>
                </ul>

                            <h2>"Être prévenu des parutions"</h2>
                <p>
                    "Ce réglage est "<strong>"fermé par défaut"</strong>". Tant qu'il l'est, rien "
                    "ne quitte votre appareil : vous êtes prévenu d'un livre, d'un chapitre ou "
                    "d'un terme qui paraît à la prochaine ouverture de l'app, ou lorsque iOS la "
                    "réveille."
                </p>
                <p>
                    "Si vous l'activez, l'app envoie à La Bible ONT "<strong>"un identifiant "
                    "d'appareil fourni par Apple"</strong>" — un jeton, et rien d'autre. Il permet "
                    "à Apple de vous livrer une notification au moment où un texte paraît."
                </p>
                <p>
                    "Nous savons ce que cette donnée dit de vous. Un jeton conservé chez nous "
                    "signifie qu'un appareil lit une Bible, ce qui relève de l'article 9 du "
                    "RGPD. C'est irréductible : sans ce jeton, aucune notification n'est "
                    "possible. Ce qui restait décidable, nous l'avons décidé."
                </p>
                <ul>
                    <li>
                        "Le jeton n'est "<strong>"rattaché à aucun compte"</strong>". Il n'existe "
                        "aucun moyen, chez nous, de relier un appareil à une personne."
                    </li>
                    <li>
                        "Rien de ce que vous lisez n'est transmis — ni le passage, ni l'heure, "
                        "ni la fréquence. La notification est "<strong>"la même pour tout le "
                        "monde"</strong>" : il n'y a pas de ciblage, et il ne peut pas y en avoir."
                    </li>
                    <li>
                        "Le jeton est rangé sous son empreinte, jamais sous un identifiant de "
                        "personne, et il "<strong>"s'efface tout seul"</strong>" après un an sans "
                        "signe de vie."
                    </li>
                    <li>
                        "Couper le réglage "<strong>"l'efface de nos serveurs"</strong>" : l'app "
                        "le retire avant de se désabonner d'Apple. Désinstaller l'app suffit "
                        "aussi — Apple nous signale l'appareil comme injoignable, et nous le "
                        "supprimons."
                    </li>
                </ul>
                <p>
                    "La base légale est "<strong>"votre consentement"</strong>", donné en activant "
                    "le réglage et retiré en le coupant. La livraison passe par Apple (APNs) ; le "
                    "jeton, lui, est conservé sur nos serveurs en Europe."
                </p>

    <h2>"Les rapports d'erreur"</h2>
                <p>
                    "Quand l'app rencontre une panne, elle envoie un rapport technique à Sentry, "
                    "dont les serveurs sont en Allemagne. Ce rapport contient la pile d'appels, "
                    "le modèle d'appareil et la version du système. Il ne contient ni le texte "
                    "que vous lisiez, ni vos notes."
                </p>

                <h2>"Vos droits"</h2>
                <p>
                    "Vous pouvez demander l'accès à vos données, leur rectification, leur "
                    "effacement, ou leur export dans un format lisible. Supprimer votre compte "
                    "depuis l'app efface les données synchronisées ; celles restées sur votre "
                    "appareil disparaissent avec l'app."
                </p>
                <p>
                    "Pour toute demande : "
                    <Lien href=format!("mailto:{CONTACT}")>{CONTACT}</Lien>
                    ". Vous pouvez aussi saisir la CNIL."
                </p>

                <h2>"Les changements"</h2>
                <p>
                    "Toute modification de ce document est visible dans l'historique public du "
                    "dépôt, avec sa date et sa raison. La date en haut de cette page est celle "
                    "de la dernière."
                </p>
            </PageLegale>
        }
}
