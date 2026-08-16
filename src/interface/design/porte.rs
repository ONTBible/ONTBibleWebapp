use leptos::prelude::*;

/// Le seuil de l'accueil — deux vantaux qui s'écartent, et l'on passe.
///
/// L'ouverture dit « Entrer » depuis le premier jour, et le bouton sautait à
/// une ancre. Le mot promettait un seuil ; la page n'en franchissait aucun.
///
/// ## Derrière la porte, il y a le contenu — pas un décor
///
/// Le premier montage posait la scène **avant** le bloc du verset et lui
/// donnait un décor à elle. On ouvrait donc sur une image, puis on arrivait sur
/// le verset : deux temps là où il n'en fallait qu'un, et la porte ne donnait
/// sur rien.
///
/// Elle **enveloppe** maintenant ce qu'elle cache. Ce qu'on voit à travers la
/// fente est le bloc lui-même, à sa vraie place dans le document — dans le HTML
/// du serveur, lisible par un moteur, lisible par un lecteur d'écran, et
/// simplement là quand la porte n'existe pas. C'est ce qui rend l'ornement
/// acceptable : il ne s'interpose devant rien qu'il n'ait d'abord porté.
///
/// ## Le clic et le défilement jouent la même chose
///
/// La scène est pilotée par la position de défilement. Le clic sur « Entrer »
/// ne fait donc rien d'autre que **déplacer le défilement** — il n'y a pas une
/// seconde animation à tenir d'accord avec la première.
///
/// Le saut d'ancre du navigateur y suffisait presque, `scroll-behavior: smooth`
/// étant déjà posé sur `html`. Presque : sa durée est fixe et courte — trois à
/// cinq dixièmes de seconde quelle que soit la distance — et la porte s'ouvrait
/// d'un claquement. Un seuil se franchit, il ne se traverse pas au pas de
/// course. D'où [`traverser`], qui n'existe que pour donner sa durée à un
/// mouvement que le navigateur ne sait pas ralentir.
///
/// La forme est dans `style/main.css`, sous « Le seuil », y compris le repli et
/// la garde de mouvement réduit. Ce fichier ne pose que la structure.
#[component]
pub fn Porte(
    /// L'ancre du bloc enveloppé — c'est vers elle que pointe « Entrer ».
    #[prop(into)]
    id: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div id=id class="seuil">
            <div class="seuil-etape">
                // Le fond, c'est-à-dire le contenu. Il n'y a pas de décor ici :
                // ce qu'on voit par la fente est ce qu'on est venu chercher.
                <div class="seuil-fond">{children()}</div>

                // Et la porte par-dessus, qui ne cache rien de durable : elle
                // ne porte aucun contenu, aucun élément focalisable, et ne
                // prend pas le clic. Un lecteur d'écran traverse le bloc sans
                // jamais la rencontrer.
                <div aria-hidden="true" class="seuil-lueur"></div>

                // Le portique — tout ce qui est bâti, et qui avance d'un bloc.
                //
                // Le mur grandit avec la caméra ; si les battants ne
                // grandissaient pas avec lui, une bande de fond s'ouvrirait
                // entre le linteau et eux dès le premier dixième de course. Ce
                // qui se tient doit avancer ensemble, donc être enveloppé
                // ensemble.
                <div aria-hidden="true" class="seuil-portique">
                    <div class="seuil-vantail seuil-vantail--g"></div>
                    <div class="seuil-vantail seuil-vantail--d"></div>

                    // Le mur percé d'une arche. Il se pose **au-dessus** des
                    // battants et non autour : c'est son ombre portée qui peint
                    // la nuit, et un battant qui s'ouvre disparaît derrière lui,
                    // comme une porte dans son tableau. Il porte aussi le tympan
                    // et le linteau, calés sur l'arche sans rien à accorder.
                    <div class="seuil-embrasure"></div>
                </div>
            </div>
        </div>
    }
}

/// Franchir le seuil à la vitesse d'un seuil.
///
/// Rend le gestionnaire de clic à poser sur le bouton qui pointe `#<ancre>`.
///
/// ## Ce qu'il corrige, et rien d'autre
///
/// Le saut d'ancre natif marche déjà, et il glisse — `scroll-behavior: smooth`
/// est posé sur `html`. Mais sa durée est **fixe et courte**, indépendante de
/// la distance : la scène du seuil se traversait en quatre dixièmes de seconde
/// et la porte claquait. Aucun réglage CSS ne la gouverne.
///
/// Il ne fait donc que déplacer le défilement, sur une durée choisie. La scène
/// reste pilotée par la position, comme au doigt — il n'y a pas deux animations
/// à tenir d'accord.
///
/// ## Il se désiste quand il n'y a pas de seuil
///
/// Si la scène ne dépasse pas la fenêtre, c'est qu'il n'y a pas de porte :
/// navigateur sans animations au défilement, ou lecteur qui a demandé moins de
/// mouvement. Il rend alors la main **sans empêcher le comportement par
/// défaut**, et le navigateur fait son saut d'ancre.
///
/// C'est une mesure et non une requête média, et c'est ce qui la rend juste :
/// elle constate l'état réel de la page au lieu de rejouer le raisonnement de
/// la feuille de style. Le jour où la garde du CSS change, ceci suit sans qu'on
/// y touche — deux endroits qui décident la même chose finissent toujours par
/// en décider deux différentes.
pub fn traverser(ancre: &'static str) -> impl Fn(leptos::ev::MouseEvent) + Clone + 'static {
    move |evenement: leptos::ev::MouseEvent| {
        let _ = (&evenement, ancre);
        #[cfg(feature = "hydrate")]
        traversee::lancer(&evenement, ancre);
    }
}

/// La durée d'un seuil.
///
/// Quatre dixièmes de seconde — ce que donne le navigateur — se lisent comme un
/// claquement. Trois secondes se lisent comme une panne. Un seuil se franchit
/// posément : on voit la fente s'ouvrir, la lumière venir, et l'on est dedans.
#[cfg(feature = "hydrate")]
const DUREE: f64 = 1600.0;

#[cfg(feature = "hydrate")]
mod traversee {
    use leptos::prelude::*;

    pub fn lancer(evenement: &leptos::ev::MouseEvent, ancre: &'static str) {
        let Some(fenetre) = web_sys::window() else {
            return;
        };
        let Some(element) = document().get_element_by_id(ancre) else {
            return;
        };

        // La scène ne dépasse pas la fenêtre : il n'y a pas de porte à
        // traverser, donc rien à ralentir. On laisse le navigateur sauter.
        if arrivee(&fenetre, &element).is_none() {
            return;
        }

        evenement.prevent_default();

        let depart = fenetre.scroll_y().unwrap_or(0.0);
        let Some(horloge) = fenetre.performance() else {
            return;
        };
        pas(ancre, depart, horloge.now());
    }

    /// Où il faut être quand la porte est grande ouverte.
    ///
    /// `contain 100%` : le bas de la scène contre le bas de la fenêtre. Rend
    /// `None` quand la scène ne dépasse pas la fenêtre — il n'y a alors pas de
    /// porte, et rien à traverser.
    fn arrivee(fenetre: &web_sys::Window, element: &web_sys::Element) -> Option<f64> {
        let cadre = element.get_bounding_client_rect();
        let hauteur = fenetre.inner_height().ok()?.as_f64()?;
        let course = cadre.height() - hauteur;
        (course > 1.0).then(|| fenetre.scroll_y().unwrap_or(0.0) + cadre.top() + course)
    }

    /// La cible est **recalculée à chaque image**, et c'est ce qui rend la
    /// traversée solide sur un téléphone.
    ///
    /// Safari rétracte sa barre d'adresse dès qu'on défile. Le Hero est mesuré
    /// en `dvh` — la règle du site — donc il **grandit** à ce moment-là, et
    /// tout ce qui est en dessous descend d'une centaine de points. Une cible
    /// calculée une fois au clic devient alors fausse en plein vol : la fin du
    /// mouvement tombe cent points trop haut, et la porte s'arrête avant
    /// d'être ouverte.
    ///
    /// Relue à chaque image, elle suit la mise en page. Le mouvement reste
    /// linéaire — c'est la destination qui bouge, pas la vitesse — et il se
    /// corrige tout seul au lieu de se casser.
    fn pas(ancre: &'static str, depart: f64, debut: f64) {
        let Some(fenetre) = web_sys::window() else {
            return;
        };
        let Some(horloge) = fenetre.performance() else {
            return;
        };
        let Some(element) = document().get_element_by_id(ancre) else {
            return;
        };
        let Some(arrivee) = arrivee(&fenetre, &element) else {
            return;
        };

        // **Linéaire, et il faut qu'il le soit.**
        //
        // Une cubique en S était en place, au motif qu'un mouvement à vitesse
        // constante « se lit comme un mécanisme ». C'est vrai d'une animation
        // qu'on regarde. Ça ne l'est pas ici : la porte n'est pas animée, elle
        // est **pilotée par la position**. Toute inflexion de la vitesse du
        // défilement devient donc une inflexion de l'ouverture — la porte ne
        // bougeait presque pas au départ, se précipitait au milieu, et rampait
        // à la fin.
        //
        // L'adoucissement d'un défilement et l'adoucissement d'une porte sont
        // deux décisions distinctes, et les superposer revient à composer deux
        // courbes dont on n'a choisi ni l'une ni l'autre. La courbe de la porte
        // est déjà dans `style/main.css` — la rotation, l'échelle à la
        // puissance quatre, les rais en parabole. Le défilement, lui, ne fait
        // que dérouler le temps.
        let avancement = ((horloge.now() - debut) / super::DUREE).clamp(0.0, 1.0);
        poser(&fenetre, depart + (arrivee - depart) * avancement);

        if avancement < 1.0 {
            request_animation_frame(move || pas(ancre, depart, debut));
        }
    }

    /// Poser le défilement à une position, **sèchement**.
    ///
    /// `window.scrollTo(x, y)` hérite du `scroll-behavior` de la feuille — et
    /// `html` porte `smooth`. Chaque appel relançait donc un défilement doux du
    /// navigateur vers la position demandée, soixante fois par seconde : deux
    /// animations sur la même page, chacune corrigeant l'autre.
    ///
    /// Ça ne se lit pas comme un ralenti, ça se lit comme des à-coups. Mesuré
    /// sur son enregistrement : pendant la traversée, des images perdues et des
    /// sauts où l'écran change presque entièrement d'une image à l'autre. On
    /// cherche alors la latence du côté du rendu — et le rendu n'y était pour
    /// rien.
    ///
    /// `behavior: instant` coupe la seconde animation. La position demandée est
    /// la position obtenue.
    fn poser(fenetre: &web_sys::Window, y: f64) {
        let options = web_sys::ScrollToOptions::new();
        options.set_left(0.0);
        options.set_top(y);
        options.set_behavior(web_sys::ScrollBehavior::Instant);
        fenetre.scroll_to_with_scroll_to_options(&options);
    }
}
