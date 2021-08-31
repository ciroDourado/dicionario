pub struct Silabas {
    silabas  : Vec<String>,
    formatada: String
} // struct Silabas


use super::silabar::separar;
use super::silabar::formatar;


impl Silabas {
    pub fn a_partir_de(termo: &str) -> Self {
        let silabas   = separar (termo);
        let formatada = formatar(&silabas);

        Silabas { silabas, formatada }
    } // a_partir_de
} // impl Silabas


use std::fmt::Formatter;
use std::fmt::Display;
use std::fmt::Result as DisplayResult;


impl Display for Silabas {
    fn fmt(&self, f: &mut Formatter<'_>)
        -> DisplayResult
    {
        write!(f, "{}", self.formatada)
    } // fmt
} // impl Display for Palavra
