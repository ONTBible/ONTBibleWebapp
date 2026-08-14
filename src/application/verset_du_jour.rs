//! Le cas d'usage : quel verset aujourd'hui.
//!
//! Il ne lit ni fichier ni horloge — il reçoit les deux par leurs ports. C'est
//! ce qui permet de vérifier « quel verset le 12 août ? » sans attendre le
//! 12 août.

use crate::application::ports::{Horloge, Vivier};
use crate::domaine::verset_du_jour::indice;
use crate::domaine::vivier::VersetQuotidien;

pub struct VersetDuJour<'a> {
    horloge: &'a dyn Horloge,
    vivier: &'a dyn Vivier,
}

impl<'a> VersetDuJour<'a> {
    pub fn new(horloge: &'a dyn Horloge, vivier: &'a dyn Vivier) -> Self {
        Self { horloge, vivier }
    }

    /// Le verset d'aujourd'hui.
    ///
    /// `None` sur un vivier vide — ce qui n'arrive que si le pipeline n'a
    /// encore verrouillé aucune unité. La page se tait alors, plutôt que
    /// d'annoncer un verset qui n'existe pas.
    pub fn aujourd_hui(&self) -> Option<&'a VersetQuotidien> {
        let versets = self.vivier.versets();
        if versets.is_empty() {
            return None;
        }
        versets.get(indice(self.horloge.jour(), versets.len()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct HorlogeFigee(i64);
    impl Horloge for HorlogeFigee {
        fn jour(&self) -> i64 {
            self.0
        }
    }

    struct VivierFixe(Vec<VersetQuotidien>);
    impl Vivier for VivierFixe {
        fn versets(&self) -> &[VersetQuotidien] {
            &self.0
        }
    }

    fn vivier(n: u32) -> VivierFixe {
        VivierFixe(
            (0..n)
                .map(|i| VersetQuotidien {
                    livre: "bereshit".into(),
                    unite: "bereshit-1".into(),
                    numero: i + 1,
                    renvoi: format!("Bereshit 1:{}", i + 1),
                    texte: format!("verset {i}"),
                })
                .collect(),
        )
    }

    #[test]
    fn un_vivier_vide_ne_rend_rien() {
        let vide = VivierFixe(Vec::new());
        assert!(VersetDuJour::new(&HorlogeFigee(20_678), &vide)
            .aujourd_hui()
            .is_none());
    }

    #[test]
    fn le_meme_jour_rend_toujours_le_meme_verset() {
        // La propriété qui compte : trois lectures d'un même jour — la page,
        // un rechargement, un partage — doivent s'accorder.
        let v = vivier(251);
        let service = VersetDuJour::new(&HorlogeFigee(20_678), &v);
        let a = service.aujourd_hui().unwrap();
        let b = service.aujourd_hui().unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn le_lendemain_change_de_verset() {
        let v = vivier(251);
        let hier = HorlogeFigee(20_678);
        let demain = HorlogeFigee(20_679);
        assert_ne!(
            VersetDuJour::new(&hier, &v).aujourd_hui(),
            VersetDuJour::new(&demain, &v).aujourd_hui()
        );
    }

    #[test]
    fn le_chemin_pointe_le_verset_et_non_l_unite() {
        let v = vivier(1);
        let verset = VersetDuJour::new(&HorlogeFigee(0), &v)
            .aujourd_hui()
            .unwrap();
        assert_eq!(verset.chemin(), "/fr/lire/bereshit/bereshit-1?v=1");
    }
}
