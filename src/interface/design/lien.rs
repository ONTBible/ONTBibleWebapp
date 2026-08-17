use leptos::prelude::*;
use leptos_router::components::A;

/// Un lien **dans le texte courant**.
///
/// ## Pourquoi il a fallu le faire
///
/// `style/main.css` porte une règle `a` qui décide tout du soulignement — sa
/// couleur (l'or à 55 %), son décalage, son épaisseur, et son passage à l'or
/// plein au survol. Elle est écrite comme si le trait existait déjà ; son
/// commentaire le dit en toutes lettres : « un lien déjà souligné ne saute pas,
/// il se confirme ».
///
/// Il n'existait pas. Le preflight de Tailwind pose `text-decoration: inherit`
/// sur les ancres, ce qui écrase le soulignement que le navigateur donne par
/// défaut — et une couleur de décoration ne dessine rien quand il n'y a pas de
/// décoration. Les liens de prose du site étaient donc **de la prose**, sans un
/// signe qui dise qu'on peut les toucher.
///
/// Le défaut est du genre le plus silencieux : la page s'affiche, le lien
/// fonctionne pour qui le trouve, et personne ne le trouve. C'est le même
/// silence que la mesure de `Bloc` ou que l'échelle qui s'inversait sous
/// 1024 px.
///
/// ## Pourquoi un composant, et pas une règle globale
///
/// Rendre le trait à **toutes** les ancres le donnerait aussi aux
/// intraduisibles — chaque mot d'or du corpus mène à sa fiche, et un chapitre
/// entier se retrouverait souligné mot après mot. Il le donnerait de même aux
/// unités du sommaire, aux cartes, au fil d'Ariane : autant de liens qui disent
/// déjà ce qu'ils sont par leur forme.
///
/// Le trait n'appartient qu'au lien **noyé dans une phrase**, le seul que rien
/// d'autre ne distingue. C'est ce cas-là que ce composant nomme.
///
/// La classe se réduit à `underline` : tout le reste est déjà dans la feuille,
/// et le redire ici ferait deux endroits à tenir d'accord.
///
/// ## Trois destinations, trois balises
///
/// Un chemin du site passe par le routeur — sinon la page entière se recharge
/// pour un lien interne. Une ancre reste une ancre : la donner au routeur lui
/// ferait chercher une route nommée `#la-beta`. Et une adresse hors du site
/// prend `noopener`, qui coupe l'accès de la page ouverte à celle-ci.
#[component]
pub fn Lien(#[prop(into)] href: String, children: Children) -> impl IntoView {
    if href.starts_with('/') {
        view! { <A href=href attr:class="underline">{children()}</A> }.into_any()
    } else if href.starts_with('#') {
        view! { <a href=href class="underline">{children()}</a> }.into_any()
    } else {
        view! { <a href=href rel="noopener" class="underline">{children()}</a> }.into_any()
    }
}
