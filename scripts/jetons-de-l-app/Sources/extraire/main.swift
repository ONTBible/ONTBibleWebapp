import SwiftUI
import ONTKit
import ONTDesignSystem

// On demande à leur propre code ce que vaut chaque rôle, dans chaque thème.
// Rien n'est recopié : si une couleur bouge chez eux, cette sortie bouge.
func hexa(_ couleur: Color) -> String {
    let natif = NSColor(couleur).usingColorSpace(.sRGB) ?? .black
    let r = Int((natif.redComponent * 255).rounded())
    let v = Int((natif.greenComponent * 255).rounded())
    let b = Int((natif.blueComponent * 255).rounded())
    let a = natif.alphaComponent
    return a < 0.999
        ? String(format: "#%02X%02X%02X%02X", r, v, b, Int((a * 255).rounded()))
        : String(format: "#%02X%02X%02X", r, v, b)
}

let themes: [(String, ReadingTheme)] = [
    ("parchemin", .parchment), ("clair", .light),
    ("sombre", .dark), ("mystique", .mystique),
]

let roles: [(String, (ReadingTheme) -> Color)] = [
    ("background", ONTColors.background), ("ink", ONTColors.ink),
    ("inkStrong", ONTColors.inkStrong), ("inkSoft", ONTColors.inkSoft),
    ("accent", ONTColors.accent), ("accentuation", ONTColors.accentuation),
    ("shem", ONTColors.shem), ("renvoi", ONTColors.renvoi),
    ("surface", ONTColors.surface), ("separator", ONTColors.separator),
    ("brandInk", ONTColors.brandInk), ("onBrand", ONTColors.onBrand),
    ("onBrandAccent", ONTColors.onBrandAccent),
    ("accentSurSurlignage", ONTColors.accentSurSurlignage),
    ("danger", ONTColors.danger), ("dangerSurface", ONTColors.dangerSurface),
    ("succes", ONTColors.succes), ("succesSurface", ONTColors.succesSurface),
    ("avertissement", ONTColors.avertissement),
    ("avertissementSurface", ONTColors.avertissementSurface),
]

// Les cinq surlignages. Ils sont **fonction du thème** chez eux — c'est une
// correction qu'ils ont faite et que le site n'avait pas : leurs pastels
// avaient été choisis pour du parchemin, et posés sur une nuit ils enfonçaient
// tous les marquages. Leur palette de nuit tient 4,6:1 sur les deux fonds
// sombres, saturation conservée pour qu'on distingue encore « le bleu » du
// « rose » à travers cinq surlignages.
let surlignages: [(String, HighlightColor)] = [
    ("surlignage-or", .gold), ("surlignage-olive", .olive),
    ("surlignage-ciel", .sky), ("surlignage-rose", .rose),
    ("surlignage-violet", .violet),
]

for (nomTheme, theme) in themes {
    for (nomRole, fonction) in roles {
        print("\(nomTheme)\t\(nomRole)\t\(hexa(fonction(theme)))")
    }
    for (nomRole, couleur) in surlignages {
        print("\(nomTheme)\t\(nomRole)\t\(hexa(ONTColors.highlight(couleur, theme)))")
    }
}
