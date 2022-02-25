use postgres::{Client, Error, Row, NoTls};

fn main() {
    let mut veritabani = db_baglanti_fn();

    match veritabani {
        Ok(veritabani) => {
           let mut sorgu = db_sorgusu_fn( veritabani);
            println!("denden")
        },
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
        "postgresql://hakanbiris:Turgutlu:45@192.168.1.30/sozluk",
        NoTls,
    )?;

    Ok(db_baglanti)

}



fn db_sorgusu_fn(mut veritabani: Client) -> Result<Vec<Row>, Error> {

    let sorgu = veritabani.query(
        "select * from kelimeler limit 25",
        &[]
    )?;

    Ok(sorgu)
}
