pub mod configuration;

pub fn list_categories<'a>(categories: &[&'a str]) {
    for (index, category) in categories.iter().enumerate() {
        println!("{}\t{}", index, category);
    }
}