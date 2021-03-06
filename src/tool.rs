use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;

//pub fn ascii_to_char(code: u8) -> char {
//    match code {
//        10 => '\n',
//        13 => ' ',
//        32 => ' ',
//        34 => '\"',
//        37 => '%',
//        35 => '#',
//        43 => '+',
//        44 => ',',
//        45 => '-',
//        46 => '.',
//        47 => '/',
//        48 => '0',
//        49 => '1',
//        50 => '2',
//        51 => '3',
//        52 => '4',
//        53 => '5',
//        54 => '6',
//        55 => '7',
//        56 => '8',
//        57 => '9',
//        58 => ':',
//        59 => ';',
//        60 => '<',
//        61 => '=',
//        62 => '>',
//        63 => '?',
//        64 => '@',
//        65 => 'A',
//        66 => 'B',
//        67 => 'C',
//        68 => 'D',
//        69 => 'E',
//        70 => 'F',
//        71 => 'G',
//        72 => 'H',
//        73 => 'I',
//        74 => 'J',
//        75 => 'K',
//        76 => 'L',
//        77 => 'M',
//        78 => 'N',
//        79 => 'O',
//        80 => 'P',
//        81 => 'Q',
//        82 => 'R',
//        83 => 'S',
//        84 => 'T',
//        85 => 'U',
//        86 => 'V',
//        87 => 'W',
//        88 => 'X',
//        89 => 'Y',
//        90 => 'Z',
//        91 => '[',
//        92 => '\\',
//        93 => ']',
//        94 => '^',
//        97 => 'a',
//        98 => 'b',
//        99 => 'c',
//        100 => 'd',
//        101 => 'e',
//        102 => 'f',
//        103 => 'g',
//        104 => 'h',
//        105 => 'i',
//        106 => 'j',
//        107 => 'k',
//        108 => 'l',
//        109 => 'm',
//        110 => 'n',
//        111 => 'o',
//        112 => 'p',
//        113 => 'q',
//        114 => 'r',
//        115 => 's',
//        116 => 't',
//        117 => 'u',
//        118 => 'v',
//        119 => 'w',
//        120 => 'x',
//        121 => 'y',
//        122 => 'z',
//        123 => '{',
//        124 => '|',
//        125 => '}',
//        126 => '~',
//        128 => '€',
//        130 => '‚',
//        148 => '”',
//        156 => 'œ',
//        160 => ' ',
//        194 => 'â',
//        226 => '|',
//        _ => '_',
//    }
//}
//建立一个专门监听的线程
pub fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });
    rx
}
pub struct Input {
    pub command: String,
    pub args: Vec<String>,
}
impl Input {
    pub fn new(input: String) -> Input {
        let pos: Vec<&str> = input.split(' ').collect();
        let length = pos.len();
        let command: String = pos[0].to_string();
        let mut args: Vec<String> = vec![];
        for item in pos.iter().take(length).skip(1) {
            args.push(item.to_string());
        }
        Input { command, args }
    }
}
