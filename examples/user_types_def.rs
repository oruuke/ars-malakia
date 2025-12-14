#[derive(Debug)]
enum BlahajSize {
    Mini = 20,
    Baby = 55,
    Normal = 100,
    Mega = 200,
}
struct Blahaj {
    size: BlahajSize,
    official: bool,
}
impl Blahaj {
    fn new(size: BlahajSize) -> Self {
        let official = match size {
            BlahajSize::Baby | BlahajSize::Normal => true,
            BlahajSize::Mini | BlahajSize::Mega => false,
        };

        Self { size, official }
    }
}

struct Fumo<'a> {
    character: &'a str,
    series: &'a str,
}
impl<'a> Fumo<'a> {
    pub fn new(character: &'a str, series: &'a str) -> Self {
        Self { character, series }
    }
}

enum Plushie<'a> {
    Blahaj(Blahaj),
    Fumo(Fumo<'a>),
}

fn main() {
    let blahaj = Plushie::Blahaj(Blahaj::new(BlahajSize::Normal));
    if let Plushie::Blahaj(b) = blahaj {
        let origin = if b.official { "from ikea" } else { "custom" };
        println!("my {}cm long blahaj was bought {}", b.size as i32, origin);
    }

    let fumo = Plushie::Fumo(Fumo::new("astolfo", "fate"));
    if let Plushie::Fumo(f) = &fumo {
        println!("and i have {} fumo from {} series", f.character, f.series);
    }
}
