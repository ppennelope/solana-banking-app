use std::io;

struct BankaHesabi {
    kullanici: String,
    bakiye: u64,
}

fn para_yatir(hesap: &mut BankaHesabi, miktar: u64) {
    if miktar == 0 {
        println!("Yatırılacak miktar 0 olamaz.");
        return;
    }

    hesap.bakiye += miktar;
    println!("{} birim yatırıldı.", miktar);
}

fn para_cek(hesap: &mut BankaHesabi, miktar: u64) {
    if miktar == 0 {
        println!("Çekilecek miktar 0 olamaz.");
    } else if miktar <= hesap.bakiye {
        hesap.bakiye -= miktar;
        println!("{} birim çekildi.", miktar);
    } else {
        println!("Yetersiz bakiye.");
    }
}

fn miktar_oku() -> u64 {
    let mut giris = String::new();

    io::stdin()
        .read_line(&mut giris)
        .expect("Giriş okunamadı.");

    match giris.trim().parse::<u64>() {
        Ok(miktar) => miktar,
        Err(_) => {
            println!("Geçersiz değer girildi.");
            0
        }
    }
}

fn main() {
    let mut hesap = BankaHesabi {
        kullanici: String::from("Mertay"),
        bakiye: 100,
    };

    println!("Kullanıcı: {}", hesap.kullanici);
    println!("Başlangıç bakiyesi: {}", hesap.bakiye);

    println!("Yatırılacak miktarı gir:");
    let yatirilan = miktar_oku();
    para_yatir(&mut hesap, yatirilan);

    println!("Çekilecek miktarı gir:");
    let cekilen = miktar_oku();
    para_cek(&mut hesap, cekilen);

    println!("Son bakiye: {}", hesap.bakiye);
}