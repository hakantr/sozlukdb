extern crate postgres;

use postgres::{Client, Error, NoTls};

fn main() {
    let veritabani = db_baglanti_fn();

    match veritabani {
        Ok(veritabani) => println!("Bağlantı sağlandı."),
        Err(e) => {
            println!("Bağlantıda hata var!");
            println!("{:?}", e.as_db_error())
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
        "postgresql://hakanbiris:Turgutlu:45@192.168.1.30/soz1luk",
        NoTls,
    )?;

    Ok(db_baglanti)
}
