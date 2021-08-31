use dicionario::Dicionario;
use dicionario::Palavra;

fn main() {
    let mut dicionario = Dicionario::novo();
    let dogue = Palavra::nova("Cachorrão", "doggo");
    let catin = Palavra::nova("Gatus", "MINHÉEEEU");

    dicionario.adicionar(dogue);
    dicionario.adicionar(catin);

    print( dicionario.consultar("Cachorrão") );
    print( dicionario.consultar("Gatus")     );
} // main


fn print<T>(nullable: Option<T>)
    where T: std::fmt::Display
{
    nullable.map(|valor| println!("\n{}", valor));
} // print
