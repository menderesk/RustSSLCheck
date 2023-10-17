use ssl_expiration2::SslExpiration;
use std::env;
use lettre::message::header::ContentType;
use lettre::{Message, SmtpTransport, Transport};

fn main() {
    let expiration = SslExpiration::from_domain_name("expired.badssl.com").expect("Domain validation has to work");
    if expiration.days() < 30 {
        let frommail = env::var("FROMMAIL").expect("$FROMMAIL is not set");
        let tomail = env::var("TOMAIL").expect("$TOMAIL is not set");
        let ssladdress = env::var("WEBADDRESS").expect("$WEBADDRESS is not set");
        println!("{ssladdress} Sertifika Süresi Dolmak Üzere!");
        let email = Message::builder()
            .from(frommail.parse().unwrap())
            .to(tomail.parse().unwrap())
            .subject("{webAddress} Sertifika Süresi Dolmak Üzere!")
            .header(ContentType::TEXT_HTML)
            .body(String::from("Sertifika Süresi dolmak üzere"))
            .unwrap();
      
        let mailer = SmtpTransport::relay("relay.server.com")
            .unwrap()
            .build();

        match mailer.send(&email) {
            Ok(_) => println!("Mail gönderildi!"),
            Err(e) => panic!("Mail gönderilemedi, hata nedeni: {e:?}"),
        }

    }
}
