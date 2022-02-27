use postgres::{Client, Error, Row, NoTls};

fn main() {

    //let mut veritabani = db_baglanti_fn();

    let mut veritabani:Option<Client> = match db_baglanti_fn() {
        Ok(veritabani) => Some(veritabani),
        Err(e) => None,
    };
    let mut satirlar:Option<Vec<Row>>=None;

    match veritabani {
        Some(mut veritabani) => {
            db_sorgusu_fn(&mut veritabani, &mut satirlar);
        },
        None => println!("Bağlantı başarısız!")
    }
    println!("hakan");
}

fn db_baglanti_fn() -> Result<Client, Error> {
    let db_baglanti = Client::connect(
        "postgresql://hakanbiris:Turgutlu:45@192.168.1.30/sozluk",
        NoTls,
    )?;

    Ok(db_baglanti)
}

fn db_sorgusu_fn(veritabani: &mut Client, satirlar: &mut Option<Vec<Row>>){
    *satirlar = match veritabani.query("select * from kelimeler limit 25",&[]){
        Ok(T) => {Some(T)},
        Err(e) =>{None},
    };
}

