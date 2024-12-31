use std::env;
use std::fs::{self, File};
use std::io::{self, Write};

/// Aplica algoritmul lui Caesar pe un text dat.
fn caesar_cipher(text: &str, shift: i32) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let shifted = (c as u8 - first + (shift as u8) % 26) % 26 + first;
                shifted as char
            } else {
                c
            }
        })
        .collect()
}

fn main() -> io::Result<()> {
    // Preluăm argumentele din linia de comandă
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Utilizare: {} <fisier_intrare> <fisier_iesire> <shift>", args[0]);
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];
    let shift: i32 = args[3].parse().expect("Shift-ul trebuie să fie un număr întreg!");

    // Citește conținutul fișierului de intrare
    let input_text = fs::read_to_string(input_file).expect("Nu s-a putut citi fișierul de intrare!");

    // Codifică textul folosind algoritmul lui Caesar
    let encoded_text = caesar_cipher(&input_text, shift);

    // Scrie textul codificat în fișierul de ieșire
    let mut output = File::create(output_file)?;
    write!(output, "{}", encoded_text)?;

    println!("Codificare completă! Textul a fost salvat în '{}'.", output_file);

    Ok(())
}
