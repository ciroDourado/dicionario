extern crate unidecode;
use unidecode::unidecode;

pub fn normalizar(palavra: &str) -> String {
    unidecode(palavra).to_lowercase()
} // normalizar
