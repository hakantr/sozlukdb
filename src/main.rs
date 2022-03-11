use postgres::{Client, Error, NoTls, Row};

fn main() {
    let mut veritabani: Option<Client> = None;
    let mut satirlar: Option<Vec<Row>> = None;

    db_baglantı_fn(&mut veritabani);
    match veritabani {
        Some(mut veritabani) => {
            db_sorgusu_fn(&mut veritabani, &mut satirlar);
        }
        None => println!("Bağlantı başarısız!"),
    }

    match satirlar {
        Some(mut satirlar) => {
            for i in satirlar {
                let soz: &str = i.get("sozcuk");
                println!("{}", soz);
            }
        }
        None => println!("Tablo bulunamadı."),
    }
}

fn db_baglantı_fn(veritabani: &mut Option<Client>) {
    *veritabani = match Client::connect(
        "postgresql://hakanbiris:Turgutlu:45@192.168.1.30/sozluk",
        NoTls,
    ) {
        Ok(t) => Some(t),
        Err(e) => None,
    };
}

fn db_sorgusu_fn(veritabani: &mut Client, satirlar: &mut Option<Vec<Row>>) {
    *satirlar = match veritabani.query("select id, sozcuk, kok from kelimeler", &[]) {
        Ok(t) => Some(t),
        Err(e) => None,
    };
}
