use hyphenation::Hyphenator;
use hyphenation::Standard;
use hyphenation::Language;
use hyphenation::Load;
use hyphenation::Iter;

pub fn separar(termo: &str) -> Vec<String> {
    Standard::from_embedded(Language::Portuguese)
        .map(|regras  | regras.hyphenate(termo))
        .map(|unidades| unidades.iter().collect())
        .unwrap_or_default()
} // separar

pub fn formatar(silabas: &Vec<String>) -> String {
    silabas.concat()
} // formatar
