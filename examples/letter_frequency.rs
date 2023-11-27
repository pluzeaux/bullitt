use bullitt::letter_frequency::frequency;
use std::collections::HashMap;

fn main() {
    const ODE_AN_DIE_FREUDE: [&str; 8] = [
        "Freude schöner Götterfunken",
        "Tochter aus Elysium,",
        "Wir betreten feuertrunken,",
        "Himmlische, dein Heiligtum!",
        "Deine Zauber binden wieder",
        "Was die Mode streng geteilt;",
        "Alle Menschen werden Brüder,",
        "Wo dein sanfter Flügel weilt.",
    ];

    let mut res: HashMap<String, u32> = HashMap::new();

    for s in ODE_AN_DIE_FREUDE {
        frequency(s, &mut res);
    }

    for (k, v) in res {
        println!("key: {}, value: {}", k, v)
    }
}
