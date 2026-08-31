//! Le domaine — ce que le site sait de La Bible ONT, indépendamment de tout.
//!
//! Cette couche ne dépend de **rien** : ni du serveur, ni du navigateur, ni
//! d'une bibliothèque de dates, ni du format des fichiers que produit le
//! pipeline. Elle compile aussi bien vers `wasm32` que vers Lambda, et c'est
//! la condition pour qu'un même calcul donne le même résultat des deux côtés.
//!
//! La règle qui la tient : elle ne fait entrer que des types qu'elle définit
//! elle-même ou que la bibliothèque standard fournit. Tout ce qui touche à
//! l'extérieur — lire un fichier, connaître l'heure — appartient aux couches
//! au-dessus, qui lui passent le résultat.

pub mod compte;
pub mod corpus;
pub mod lecture;
pub mod recherche;
pub mod selection;
pub mod surlignage;
pub mod texte;
pub mod verset_du_jour;
pub mod vivier;
