//! Le vivier du verset du jour.
//!
//! Le type est **plat** — pas d'arbre d'inline, seulement le corps de la
//! traduction. C'est un choix de l'app, repris ici : un verset du jour se lit
//! d'une traite, et les gloses de l'ONT font parfois quarante mots.
//!
//! Le vivier ne retient que les **unités verrouillées** (§12) : un brouillon
//! ne fait pas référence. C'est le pipeline qui applique cette règle, pas
//! l'affichage — sinon elle finirait appliquée à deux endroits sur trois.

/// Un verset du vivier quotidien.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersetQuotidien {
    pub livre: String,
    pub unite: String,
    pub numero: u32,
    /// Le renvoi affichable — « Bereshit 1:1 ».
    pub renvoi: String,
    /// Le corps de la traduction, sans gloses ni translittérations.
    pub texte: String,
}

impl VersetQuotidien {
    /// Le lien vers le passage, dans la liseuse.
    ///
    /// Le segment de langue est délibéré (§4) : il épargne une migration le
    /// jour d'une édition anglaise, et il ne coûte que trois caractères.
    pub fn chemin(&self) -> String {
        format!("/fr/lire/{}/{}?v={}", self.livre, self.unite, self.numero)
    }
}
