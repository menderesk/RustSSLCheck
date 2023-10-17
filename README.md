# RustSSLCheck

Basit anlamda bir hedef adresin SSL sertifika süresini kontrol eden Rust uygulaması. Eğer hedefin sertifika sona erme tarihi 30 günden az kaldı ise mail gönderimi sağlamaktadır. Microservice olarak çalıştırılması için environment değişkenlerini deklere etmeniz gerekir. 

## Build
git clone https://github.com/menderesk/RustSSLCheck.git && cd RustSSLCheck
cargo build

## Run
cargo run
