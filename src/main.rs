#[macro_use]
extern crate rust_i18n;

// Init translations for current crate.
i18n!("locales");

fn main() {
    println!("{}, Longbridge!", t!("hello"));

    //
    println!("{}", example_base::hello("Longbridge"));

    rust_i18n::set_locale("fr");
    println!("{}, Longbridge!", t!("hello"));
    println!("{}", example_base::hello("Longbridge"));
}
