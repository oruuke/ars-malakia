enum BlahajSize {
    Mini,
    Baby,
    Normal,
    Mega,
}
struct Blahaj {
    length: u16,
    from_ikea: bool,
}
impl Blahaj {
    fn new(blahaj_size: BlahajSize) -> Self {
        match blahaj_size {
            BlahajSize::Mini => Self {
                length: 20,
                from_ikea: false,
            },
            BlahajSize::Baby => Self {
                length: 55,
                from_ikea: true,
            },
            BlahajSize::Normal => Self {
                length: 100,
                from_ikea: true,
            },
            BlahajSize::Mega => Self {
                length: 200,
                from_ikea: false,
            },
        }
    }
}

struct Fumo<'a> {
    character: &'a str,
    series: &'a str,
}
impl<'a> Fumo<'a> {
    pub fn new(character: &'a str) -> Self {
        Self {
            character,
            series: match character {
                "astolfo" => "fate",
                "cirno" => "touhou",
                _ => "unknown",
            },
        }
    }
}

enum Plushie<'a> {
    Blahaj(Blahaj),
    Fumo(Fumo<'a>),
}

fn main() {
    let blahaj = Plushie::Blahaj(Blahaj::new(BlahajSize::Normal));
    if let Plushie::Blahaj(b) = &blahaj {
        let origin = if b.from_ikea { "ikea" } else { "bootleg" };
        println!("my {} blahaj is {}cm long", origin, b.length);
    }

    let fumo = Plushie::Fumo(Fumo::new("astolfo"));
    if let Plushie::Fumo(f) = &fumo {
        println!("and i have {} fumo from {} series", f.character, f.series);
    }
}
