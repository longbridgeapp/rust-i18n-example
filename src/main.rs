use rust_i18n::t;

fn main() {
    rust_i18n::set_locale("fr-FR");
    println!("{}, Longbridge!", t!("hello"));
}
