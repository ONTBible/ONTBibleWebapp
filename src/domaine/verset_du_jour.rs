//! Le choix du verset du jour.
//!
//! ## Pourquoi ce code existe en double
//!
//! L'app iOS, son widget et sa notification tombent sur le même verset le même
//! jour sans jamais se parler — parce que le choix est une **fonction de la
//! date**, et non un tirage. Si le site affiche le verset du jour, il doit
//! employer la même fonction, sinon il annoncerait un autre verset que le
//! téléphone du lecteur le même matin.
//!
//! L'original est `DailySelection`, dans
//! `ONTBibleApp/app/Packages/ONTKit/Sources/ONTKit/Reader/DailyVerse.swift`.
//! Ce module en est le portage littéral. Deux langages, donc deux
//! implémentations : c'est le seul endroit du projet où la duplication est
//! inévitable, et c'est pourquoi elle est isolée ici, pure, et éprouvée par des
//! tests qui rejouent les valeurs de l'original.
//!
//! ## Une permutation, pas un tirage
//!
//! Sur un vivier de 251 versets, un tirage ferait revenir le même verset deux
//! fois dans le mois avec quatre chances sur cinq — le paradoxe des
//! anniversaires. On avance donc d'un **pas fixe premier avec la taille du
//! vivier** : un tel pas engendre le groupe entier, donc il visite les 251
//! versets un par un avant d'en revoir un seul.
//!
//! Le pas vaut environ 0,618 × la taille — le nombre d'or, qui écarte au
//! maximum deux positions consécutives. Deux jours voisins restent donc
//! éloignés dans le corpus.

/// Le nombre d'or moins un — le rapport qui disperse le mieux deux positions
/// consécutives d'une suite additive.
const NOMBRE_DOR: f64 = 0.618_033_988_749_895;

/// L'indice du verset du jour, dans un vivier de `taille` éléments.
///
/// `numero_de_jour` est le nombre de jours écoulés depuis le 1ᵉʳ janvier 1970,
/// **dans le fuseau du lecteur**. Le domaine ne sait pas lire l'heure : c'est
/// la couche du dessus qui la lui donne, ce qui rend cette fonction pure et
/// permet de la tester sans horloge.
///
/// Rend `0` sur un vivier vide ou singleton — il n'y a alors rien à choisir.
pub fn indice(numero_de_jour: i64, taille: usize) -> usize {
    if taille <= 1 {
        return 0;
    }

    let taille_i = taille as i64;
    // Le reste de Rust suit le signe du dividende, comme celui de Swift : une
    // date d'avant 1970 donnerait un indice négatif sans ce repli.
    let position = (((numero_de_jour % taille_i) + taille_i) % taille_i) as usize;

    (position * pas(taille)) % taille
}

/// Le pas d'avancement — premier avec `taille`, donc générateur du cycle.
///
/// On part de 0,618 × la taille et on avance jusqu'au premier entier premier
/// avec elle. Le décalage est de quelques unités au plus, donc la dispersion
/// reste celle du nombre d'or.
pub fn pas(taille: usize) -> usize {
    if taille <= 2 {
        return 1;
    }
    let mut pas = ((taille as f64 * NOMBRE_DOR) as usize).max(1);
    while pgcd(pas, taille) != 1 {
        pas += 1;
    }
    pas
}

fn pgcd(a: usize, b: usize) -> usize {
    let (mut x, mut y) = (a, b);
    while y != 0 {
        let reste = x % y;
        x = y;
        y = reste;
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Les valeurs de l'original, relevées en exécutant le Swift.
    ///
    /// Ce n'est pas une propriété qu'on vérifie ici, c'est un **accord entre
    /// deux langages**. Le témoin a été produit en compilant le vrai
    /// `DailyVerse.swift` de l'app avec un calendrier en UTC, puis en lisant
    /// ses sorties. Si ce test tombe un jour, c'est que les deux
    /// implémentations ont divergé — et le site annoncerait alors un autre
    /// verset que le téléphone du lecteur, le même matin.
    ///
    /// Régénérer le témoin : compiler `DailySelection.index` avec un
    /// `Calendar` en UTC sur les mêmes `JOURS`.
    #[test]
    fn le_portage_s_accorde_avec_l_original_swift() {
        const JOURS: [i64; 8] = [0, 1, 2, 17, 20_678, 20_679, -1, -365];
        const TEMOIN: [(usize, [usize; 8]); 3] = [
            (7, [0, 4, 1, 5, 0, 4, 3, 3]),
            (251, [0, 155, 59, 125, 71, 226, 96, 151]),
            (1000, [0, 619, 238, 523, 682, 301, 381, 65]),
        ];

        for (taille, attendus) in TEMOIN {
            for (jour, attendu) in JOURS.iter().zip(attendus) {
                assert_eq!(
                    indice(*jour, taille),
                    attendu,
                    "vivier de {taille}, jour {jour}"
                );
            }
        }

        // Le pas, relevé au même endroit.
        assert_eq!(pas(7), 4);
        assert_eq!(pas(251), 155);
        assert_eq!(pas(1000), 619);
    }

    #[test]
    fn vivier_vide_ou_unique_ne_choisit_rien() {
        assert_eq!(indice(20_000, 0), 0);
        assert_eq!(indice(20_000, 1), 0);
    }

    #[test]
    fn le_pas_est_toujours_premier_avec_la_taille() {
        // La propriété qui garantit qu'aucun verset ne revient avant que tous
        // soient passés. Si elle tombe, le cycle se referme trop tôt.
        for taille in 2..=1_000 {
            assert_eq!(pgcd(pas(taille), taille), 1, "taille {taille}");
        }
    }

    #[test]
    fn aucun_verset_ne_revient_avant_que_tous_soient_passes() {
        // Le vivier réel fait 251 versets (§12 : unités verrouillées seulement).
        let taille = 251;
        let mut vus = vec![false; taille];
        for jour in 0..taille as i64 {
            let i = indice(jour, taille);
            assert!(!vus[i], "le verset {i} revient au jour {jour}");
            vus[i] = true;
        }
        assert!(vus.into_iter().all(|v| v), "des versets n'ont jamais été vus");
    }

    #[test]
    fn deux_jours_voisins_sont_eloignes_dans_le_corpus() {
        // Le but du nombre d'or : hier et aujourd'hui ne doivent pas être deux
        // versets consécutifs du même chapitre.
        let taille = 251;
        for jour in 0..200 {
            let a = indice(jour, taille) as i64;
            let b = indice(jour + 1, taille) as i64;
            let ecart = (a - b).abs().min(taille as i64 - (a - b).abs());
            assert!(ecart > 10, "jour {jour} : écart de {ecart} seulement");
        }
    }

    #[test]
    fn une_date_d_avant_1970_ne_deborde_pas() {
        // Personne ne consultera le site en 1969, mais un fuseau mal converti
        // peut produire un numéro négatif — et un indice négatif paniquerait.
        assert!(indice(-1, 251) < 251);
        assert!(indice(-100_000, 251) < 251);
    }
}
