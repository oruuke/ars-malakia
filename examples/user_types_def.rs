fn main() {
    // struct has heterogeneous fields of other types
    struct Dimensions {
        _height: u8,
        _width: u8,
        length: u8,
    }
    struct Plush<'a> {
        kind: &'a str,
        size: Dimensions,
        colours: (&'a str, &'a str),
    }
    let size = Dimensions {
        _height: 4,
        _width: 4,
        length: 30,
    };
    let blahaj = Plush {
        kind: "blahaj",
        size: size,
        colours: ("blue", "white"),
    };
    println!(
        "my {} is {}cm long and is classic {}",
        blahaj.kind, blahaj.size.length, blahaj.colours.0
    )
}
