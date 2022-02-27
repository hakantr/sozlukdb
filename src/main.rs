use postgres::{Client, Error, Row, NoTls};

fn main() {

    //let mut veritabani = db_baglanti_fn();

    // let mut veritabani:Option<Client> = match db_baglantı_fn(&mut veritabani) {
    //     Ok(veritabani) => Some(veritabani),
    //     Err(e) => None,
    // };
    let mut veritabani:Option<Client>=None;
    let mut satirlar:Option<Vec<Row>>=None;

    db_baglantı_fn(&mut veritabani);
    match veritabani {
        Some(mut veritabani) => {
            db_sorgusu_fn(&mut veritabani, &mut satirlar);
        },
        None => println!("Bağlantı başarısız!")
    }

    for i in Some(satirlar) {
        let soz = i.get(1);
        println!("{:?}", soz);
    }
}

fn db_baglantı_fn(veritabani: &mut Option<Client>){

    *veritabani = match Client::connect("postgresql://hakanbiris:Turgutlu:45@192.168.1.30/sozluk", NoTls )
    {
        Ok(T) => {Some(T)},
        Err(e) => {None},
    };
}

// fn db_baglanti_fn() -> Result<Client, Error> {
//     let db_baglanti = Client::connect(
//         "postgresql://hakanbiris:Turgutlu:45@192.168.1.30/sozluk",
//         NoTls,
//     )?;
//
//     Ok(db_baglanti)
// }

fn db_sorgusu_fn(veritabani: &mut Client, satirlar: &mut Option<Vec<Row>>){
    *satirlar = match veritabani.query("select id, sozcuk, kok from kelimeler limit 25",&[]){
        Ok(T) => {Some(T)},
        Err(e) =>{None},
    };
}


