extern crate postgres;

use postgres::{Client, Error, NoTls};

fn main() {
    let veritabani = db_baglanti_fn();

    match veritabani {
        Ok(veritabani) => println!("bağlantı sağlandı"),
        Err(e) => {
            println!("işlem başarılı değil");
            println!("{}", e)
        }
    }

    println!("hakan");

    //    if veritabani.is_err() {
    //        println!("hatalı ");
    //    } else {
    //        println!("bağlantı sağlandı")
    //    }
}

fn db_baglanti_fn() -> Result<Client, Error> {
    let db_baglanti = Client::connect(
        "postgresql://hakanbiris:Turgutlu:45@192.168.1.30/soszluk",
        NoTls,
    )?;

    Ok(db_baglanti)
}
