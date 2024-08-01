use fry_core::process;


fn main() {
    let bytes = process(vec![]);

    let image = image::load_from_memory(&bytes).unwrap();

    image.save("output.png").unwrap();
}