use bullitt::miners::{create_map, isolate_char, populate_vec};

fn main() {
    let _ve: Vec<String> = vec![
        String::from("  *  "),
        String::from("  *  "),
        String::from("*****"),
        String::from("  *  "),
        String::from("  *  "),
    ];

    let ars = [" 2*2 ", "25*52", "*****", "25*52", " 2*2 "];

    let shape: (u32, u32) = (ars.len() as u32, ars[0].len() as u32);
    let hm = isolate_char(&ars);
    let hmr = create_map(hm);
    let pv = populate_vec(hmr, shape);

    for s in pv {
        println!("{}", s)
    }
}
