use crate::silabacao::silabas::Silabas;
use crate::normalizacao::normalizar::normalizar;

pub struct Palavra {
    pub termo  : String,
    normalizada: String,
    significado: String,
    silabas    : Silabas
} // struct Palavra


impl Palavra {
    pub fn nova(termo: &str, significado: &str)
        -> Self
    {
        Palavra {
            termo      : termo.to_string(),
            normalizada: normalizar(termo),
            significado: significado.to_string(),
            silabas    : Silabas::a_partir_de(termo)
        }
    } // nova
} // impl Palavra


use std::fmt::Formatter;
use std::fmt::Display;
use std::fmt::Result as DisplayResult;

impl Display for Palavra {
    fn fmt(&self, f: &mut Formatter<'_>)
        -> DisplayResult
    {
        let partes = [
            format!("Palavra    : {}", self.termo),
            format!("Normalizada: {}", self.normalizada),
            format!("Significado: {}", self.significado),
            format!("Silabada   : {}", self.silabas)
        ];
        write!(f, "{}", partes.join("\n"))
    }
} // impl Display for Palavra
