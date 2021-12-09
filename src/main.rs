use unicode_segmentation::UnicodeSegmentation;
use getopts::Options;
use std::env;

const MORSE: [[&str; 2]; 55] = [
    ["Aa", ".-"],
    ["Bb", "-..."],
    ["Cc", "-.-."],
    ["Dd", "-.."],
    ["Ee", "."],
    ["Ff", "..-."],
    ["Gg", "--."],
    ["Hh", "...."],
    ["Ii", ".."],
    ["Jj", ".---"],
    ["Kk", "-.-"],
    ["Ll", ".-.."],
    ["Mm", "--"],
    ["Nn", "-."],
    ["Oo", "---"],
    ["Pp", ".--."],
    ["Qq", "--.-"],
    ["Rr", ".-."],
    ["Ss", "..."],
    ["Tt", "-"],
    ["Uu", "..-"],
    ["Vv", "...-"],
    ["Ww", ".--"],
    ["Xx", "-..-"],
    ["Yy", "-.--"],
    ["Zz", "--.."],
    ["0", "-----"],
    ["1", ".----"],
    ["2", "..---"],
    ["3", "...--"],
    ["4", "....-"],
    ["5", "....."],
    ["6", "-...."],
    ["7", "--..."],
    ["8", "---.."],
    ["9", "----."],
    [".", ".-.-.-"],
    [",", "--..--"],
    ["?", "..--.."],
    ["'", ".----."],
    ["!", "-.-.--"],
    ["/", "-..-."],
    ["(", "-.--."],
    [")", "-.--.-"],
    ["&", ".-..."],
    [":", "---..."],
    [";", "-.-.-."],
    ["=", "-...-"],
    ["+", ".-.-."],
    ["-", "-....-"],
    ["_", "..--.-"],
    ["\"", ".-..-."],
    ["$", "...-..-"],
    ["@", ".--.-."],
    [" ", "/"],
];

const PROSIGN_UTF8_BEGIN: &str = "-.....";
const PROSIGN_UTF8_END: &str = ".....-";

fn grapheme_to_morse(c: &str) -> String {
    if let Some(code) = MORSE.iter().find(|x| x[0].contains(c)) {
        return code[1].to_string() + " ";
    }
    let mut morse = PROSIGN_UTF8_BEGIN.to_string() + " ";
    let grapheme_bytes = c.as_bytes();
    grapheme_bytes.iter().for_each(|byte| {
        for i in (0..8).rev() {
            if ((byte >> i) & 1) == 1 {
                morse += ".";
            } else {
                morse += "-";
            }
        }
        morse += " ";
    });
    morse + PROSIGN_UTF8_END
}

fn morse_to_grapheme(morse: &str) -> String {
    let parts = morse.split_whitespace().collect::<Vec<&str>>();
    if PROSIGN_UTF8_BEGIN == parts[0] {
        let mut bytes = Vec::<u8>::new();
        parts[1..parts.len()-1]
            .iter()
            .take_while(|x| *x != &PROSIGN_UTF8_END)
            .for_each(|x| {
                let mut byte = 0u8;
                for (i, c) in x.chars().enumerate() {
                    if c == '.' {
                        byte |= 1 << (7 - i);
                    }
                }
                bytes.push(byte);
            });
        std::str::from_utf8(&bytes[..]).unwrap().to_string()
    } else {
        match MORSE.iter().find(|x| x[1] == morse) {
            Some(c) => { c[0].chars().next().unwrap().to_string() },
            _ => { panic!("Unknown morse code: {}", morse) },
        }
    }
}

fn word_to_morse(word: &str) -> String {
    word
        .graphemes(true)
        .map(|c| grapheme_to_morse(c))
        .collect::<String>()
        .trim()
        .to_string()
}

fn morse_to_word(word: &str) -> String {
    let codes = word.split_whitespace().collect::<Vec<&str>>();
    let mut iter = codes.iter();
    let mut s = "".to_string();
    while let Some(code) = iter.next() {
        if code != &PROSIGN_UTF8_BEGIN {
            s += &morse_to_grapheme(code);
        } else {
            let utf8_encoded = iter.clone().take_while(|x| *x != &PROSIGN_UTF8_END).collect::<Vec<_>>();
            let mut utf_string = PROSIGN_UTF8_BEGIN.to_string() + " ";
            for u in &utf8_encoded {
                utf_string += u;
                utf_string += " ";
            }
            utf_string += PROSIGN_UTF8_END;
            s += &morse_to_grapheme(&utf_string);

            // Consume utf
            for _ in utf8_encoded {
                iter.next();
            }
            iter.next();
        }
    }

    s
}

fn usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [conversion] string", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args = env::args() .collect::<Vec<String>>();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("m", "to-morse", "Convert UTF-8 text to morse code (default)");
    opts.optflag("u", "to-utf8", "Convert morse code to UTF-8 text");
    opts.optflag("h", "help", "Print this help");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };

    if matches.opt_present("h") {
        usage(&program, opts);
        return;
    }

    let input = if !matches.free.is_empty(){
        matches.free.clone()
    } else {
        usage(&program, opts);
        return;
    };

    if matches.opt_present("u") {
        let arg_morse = input
            .join("")
            .split(" / ")
            .map(|arg| {
                morse_to_word(arg)
            }).collect::<Vec<String>>();
        println!("{}", arg_morse.join(" "));
        return;
    }

    let arg_morse = input
        .iter()
        .map(|arg| {
            arg
                .split_whitespace()
                .map(|word| word_to_morse(word))
                .collect()
        }).collect::<Vec<String>>();

    println!("{}", arg_morse.join(" / "));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_standard_morse() {
        let sos = word_to_morse("SOS");
        assert_eq!(sos, "... --- ...");

        let god = "What hath God wrought"
            .split_whitespace()
            .map(|x| word_to_morse(x))
            .collect::<Vec<String>>()
            .join(" / ");
        assert_eq!(god, ".-- .... .- - / .... .- - .... / --. --- -.. / .-- .-. --- ..- --. .... -");
    }

    #[test]
    fn test_utf8_morse() {
        let crab = word_to_morse("🦀");
        assert_eq!(crab, "-..... ....---- .--..... .-.--..- .------- .....-");
    }

    #[test]
    fn test_mixed_morse() {
        let sos_crab = word_to_morse("SOS 🦀");
        let crab_sos = morse_to_word("-..... ....---- .--..... .-.--..- .------- .....- ... --- ...");
        assert_eq!(sos_crab, "... --- ... / -..... ....---- .--..... .-.--..- .------- .....-");
        assert_eq!(crab_sos, "🦀SOS");
    }
}
