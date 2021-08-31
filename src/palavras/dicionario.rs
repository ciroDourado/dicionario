use std::collections::HashMap;
use super::palavra::Palavra;


pub struct Dicionario {
    dicionario: HashMap<String, Palavra>
} // struct Dicionario


impl Dicionario {

    pub fn novo()
        -> Self
    {
        let dicionario = HashMap::default();
        Dicionario { dicionario }
    } // novo


    pub fn adicionar(&mut self, palavra: Palavra)
    {
        match self.dicionario.contains_key(&palavra.termo) {
            false => self.adicionar_palavra(palavra) ,
            true  => self.adicionar_significado(palavra)
        }
    } // adicionar


    fn adicionar_palavra(&mut self, palavra: Palavra)
    {
        let key = palavra.termo.clone();
        self.dicionario.insert(key, palavra);
    } // adicionar_palavra


    fn adicionar_significado(&mut self, palavra: Palavra)
    {
        println!("uepaaaaa")
    } // adicionar_significado


    pub fn consultar(&self, termo: &str)
        -> Option<&Palavra>
    {
        self.dicionario.get(termo)
    } // consultar
}
