use figlet_rs::FIGfont;
use ncurses::*;
use rand::{thread_rng, Rng};
use std::env::args;
use std::time::Instant;
use unidecode::unidecode;

fn from_code_to_key(code: i32) -> Option<String> {
    let result = match code {
        32 => " ",
        33 => "!",
        34 => "\"",
        35 => "#",
        36 => "$",
        37 => "%",
        38 => "&",
        39 => "'",
        40 => "(",
        42 => "*",
        41 => ")",
        43 => "+",
        44 => ",",
        45 => "-",
        46 => ".",
        47 => "/",
        48 => "0",
        49 => "1",
        50 => "2",
        51 => "3",
        52 => "4",
        53 => "5",
        54 => "6",
        55 => "7",
        56 => "8",
        57 => "9",
        58 => ":",
        59 => ";",
        60 => "<",
        61 => "=",
        62 => ">",
        63 => "?",
        64 => "@",
        65 => "A",
        66 => "B",
        67 => "C",
        68 => "D",
        69 => "E",
        70 => "F",
        71 => "G",
        72 => "H",
        73 => "I",
        74 => "J",
        75 => "K",
        76 => "L",
        77 => "M",
        78 => "N",
        79 => "O",
        80 => "P",
        81 => "Q",
        82 => "R",
        83 => "S",
        84 => "T",
        85 => "U",
        86 => "V",
        87 => "W",
        88 => "X",
        89 => "Y",
        90 => "Z",
        91 => "[",
        92 => "\\",
        93 => "]",
        97 => "a",
        98 => "b",
        99 => "c",
        100 => "d",
        101 => "e",
        102 => "f",
        103 => "g",
        104 => "h",
        105 => "i",
        106 => "j",
        107 => "k",
        108 => "l",
        109 => "m",
        110 => "n",
        111 => "o",
        112 => "p",
        113 => "q",
        114 => "r",
        115 => "s",
        116 => "t",
        117 => "u",
        118 => "v",
        119 => "w",
        120 => "x",
        121 => "y",
        122 => "z",
        123 => "{",
        124 => "|",
        125 => "}",
        129 => "Á",
        130 => "Â",
        131 => "Ã",
        137 => "É",
        138 => "Ê",
        141 => "Í",
        142 => "Î",
        147 => "Ó",
        148 => "Ô",
        149 => "Õ",
        154 => "Ú",
        155 => "Û",
        161 => "á",
        162 => "â",
        163 => "ã",
        167 => "ç",
        168 => "Ũ",
        169 => "é",
        170 => "ê",
        173 => "í",
        174 => "î",
        179 => "ó",
        180 => "ô",
        181 => "õ",
        186 => "ú",
        188 => "Ẽ",
        187 => "û",
        189 => "ẽ",
        _ => "_",
    };

    if result == "_" {
        None
    } else {
        Some(result.to_string())
    }
}

const WORDS: &str = include_str!("words.txt");

#[derive(Debug, Clone)]
struct Character {
    text: String,
    typed: bool,
    correct: bool,
}

#[derive(Debug, Clone)]
struct Word {
    chars: Vec<Character>,
    written_chars: Vec<String>,
    extra_written_chars: Vec<String>,
    typed: bool,
    correct: bool,
}

impl From<String> for Word {
    fn from(word: String) -> Self {
        Word {
            chars: word
                .chars()
                .map(|char| Character {
                    text: char.to_string(),
                    correct: false,
                    typed: false,
                })
                .collect(),
            written_chars: Vec::with_capacity(word.len() * 2),
            extra_written_chars: Vec::with_capacity(word.len()),
            typed: false,
            correct: false,
        }
    }
}

fn make_random_text_from_words(text_words_amount: usize, words: &Vec<&str>) -> Vec<Word> {
    let mut rng = thread_rng();
    let mut result: Vec<Word> = Vec::with_capacity(text_words_amount);

    for _ in 0..text_words_amount {
        let word_index = rng.gen_range(0..words.len());
        let word = words[word_index].to_string();
        result.push(word.into());
    }

    result
}

const GREEN: i16 = 1;
const RED: i16 = 2;
const YELLOW: i16 = 3;
const CYAN: i16 = 4;

#[test]
fn word_should_count_drawn_characters_correctly() {
    let word = Word {
        chars: vec![
            Character {
                text: "r".to_string(),
                typed: false,
                correct: false,
            },
            Character {
                text: "e".to_string(),
                typed: false,
                correct: false,
            },
            Character {
                text: "l".to_string(),
                typed: false,
                correct: false,
            },
            Character {
                text: "a".to_string(),
                typed: false,
                correct: false,
            },
            Character {
                text: "ç".to_string(),
                typed: false,
                correct: false,
            },
            Character {
                text: "´".to_string(),
                typed: false,
                correct: false,
            },
            Character {
                text: "ã".to_string(),
                typed: false,
                correct: false,
            },
            Character {
                text: "o".to_string(),
                typed: false,
                correct: false,
            },
        ],
        correct: false,
        typed: false,
        written_chars: Vec::default(),
        extra_written_chars: vec![
            "b".to_string(),
            "é".to_string()
        ],
    };

    assert_eq!(word.count_drawn_characters(), 9);
}

impl Word {
    fn count_drawn_characters(&self) -> usize {
        let mut typed_text: String = "".into();
        self.chars.iter().for_each(|char| {
            if char.text != "'" && char.text != "´" && char.text != "`"
                && char.text != "~" && char.text != "¨" {
                typed_text.push_str(&unidecode(&char.text));
            }
        });
        self.extra_written_chars.iter().for_each(|char| {
            if char != "'" && char != "´" && char != "`"
                && char != "~" && char != "¨" {
                typed_text.push_str(&unidecode(&char));
            }
        });
        return typed_text.len();
    }

    fn draw(&self, win: WINDOW, current_char_index: usize, is_current: bool) {
        for (i, char) in self.chars.iter().enumerate() {
            if char.typed && char.correct {
                wattron(win, COLOR_PAIR(GREEN));
                waddstr(win, char.text.as_str());
                wattroff(win, COLOR_PAIR(GREEN));
            } else if char.typed {
                wattron(win, COLOR_PAIR(RED));
                waddstr(win, char.text.as_str());
                wattroff(win, COLOR_PAIR(RED));
            } else if i == current_char_index && is_current {
                wattron(win, A_UNDERLINE());
                waddstr(win, char.text.as_str());
                wattroff(win, A_UNDERLINE());
            } else {
                waddstr(win, char.text.as_str());
            }
        }

        for (i, char) in self.extra_written_chars.iter().enumerate() {
            if i == current_char_index && is_current {
                wattron(win, A_UNDERLINE());
                wattron(win, COLOR_PAIR(YELLOW));
                waddstr(win, char);
                wattron(win, COLOR_PAIR(YELLOW));
                wattroff(win, A_UNDERLINE());
            } else {
                wattron(win, COLOR_PAIR(YELLOW));
                waddstr(win, char);
                wattroff(win, COLOR_PAIR(YELLOW));
            }
        }
    }

    fn update(&mut self, character: &str, current_char_index: usize) {
        if current_char_index < self.chars.len() {
            self.chars[current_char_index].typed = true;
            self.chars[current_char_index].correct =
                character == self.chars[current_char_index].text;
        }
        self.written_chars.push(character.to_string());
        if current_char_index >= self.chars.len() {
            self.extra_written_chars.push(character.to_string());
        }
    }

    fn is_correct(&self) -> bool {
        if self.written_chars.len() != self.chars.len() {
            return false;
        }

        let mut correct = true;
        for (char, respective_char) in self.chars.iter().zip(&self.written_chars) {
            if &char.text != respective_char {
                correct = false;
                break;
            }
        }

        correct
    }
}

#[derive(Debug, Clone)]
struct TypingResults {
    wpm: f32,
    raw_wpm: f32,
    accuracy: f32,
    characters_typed: usize,
    correct_characters: usize,
    incorrect_characters: usize,
    time_elapsed: std::time::Duration,
}

fn compute_typing_results(start: Instant, words: Vec<Word>) -> TypingResults {
    let elapsed_time = start.elapsed().as_secs_f32();

    let correct_characters = words
        .iter()
        .map(|w| w.chars.iter().filter(|c| c.correct).count())
        .sum::<usize>();
    let incorrect_characters = words
        .iter()
        .map(|w| w.chars.iter().filter(|c| !c.correct).count() + w.extra_written_chars.len())
        .sum::<usize>();
    let characters_typed = incorrect_characters + correct_characters;

    let raw_wpm = (characters_typed as f32 / 5.0f32) / (elapsed_time / 60f32);
    let accuracy = correct_characters as f32 / characters_typed as f32;
    let wpm = raw_wpm * accuracy;

    TypingResults {
        wpm,
        raw_wpm,
        accuracy,
        time_elapsed: start.elapsed(),
        characters_typed,
        correct_characters,
        incorrect_characters,
    }
}

trait TypingTestText {
    fn draw(
        &self,
        win: WINDOW,
        current_char_index: usize,
        current_word_index: usize,
        win_width: i32,
    ) -> ();

    fn count_typed_characters(&self) -> usize;
}

impl TypingTestText for Vec<Word> {
    fn draw(
        &self,
        win: WINDOW,
        current_char_index: usize,
        current_word_index: usize,
        win_width: i32,
    ) -> () {
        let mut x = 1;

        for (i, word) in self.iter().enumerate() {
            let drawn_characters = word.count_drawn_characters() as i32;
            if drawn_characters + x >= win_width {
                x = 1;
                waddstr(win, "\n");
            }
            x += drawn_characters as i32;
            word.draw(win, current_char_index, i == current_word_index);
            if i < self.len() - 1 {
                let next_word = &self[i + 1];
                let next_drawn_characters =
                    next_word.count_drawn_characters() as i32;
                if next_drawn_characters + x < win_width {
                    x += 1;
                    waddstr(win, " ");
                }
            }
        }
    }

    fn count_typed_characters(&self) -> usize {
        self.iter().map(|w| w.written_chars.len()).sum()
    }
}

fn main() {
    let args = args().collect::<Vec<String>>();
    let words_amount = args.get(1).unwrap_or(&"50".to_string()).parse::<i32>()
        .expect("Se você quiser específicar a quantidade de palavras no teste vai precisar ser um número válido inteiro.");

    let words: Vec<&str> = WORDS.split("\n").collect();
    initscr();
    if has_colors() == false {
        endwin();
        println!("Seu terminal não suporta cor!\n");
        return;
    }

    start_color();
    init_pair(GREEN, COLOR_GREEN, COLOR_BLACK);
    init_pair(RED, COLOR_RED, COLOR_BLACK);
    init_pair(YELLOW, COLOR_YELLOW, COLOR_BLACK);
    init_pair(CYAN, COLOR_CYAN, COLOR_BLACK);

    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    let window_width = (max_x / 2).min(200);
    let window_height = max_y / 2;
    let start_x = (max_x - window_width) / 2;
    let start_y = (max_y - window_height) / 2;
    refresh();
    let win = newwin(window_height, window_width, start_y, start_x);
    let mut actual_window_height = 0;
    let mut actual_window_width = 0;
    getmaxyx(win, &mut actual_window_height, &mut actual_window_width);
    wrefresh(win);
    cbreak();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    noecho();

    let mut words = make_random_text_from_words(words_amount as usize, &words);
    let mut current_word_index: usize = 0;
    let mut current_char_index: usize = 0;

    words.draw(win, current_char_index, current_word_index, window_width);

    wattron(win, COLOR_PAIR(CYAN));
    waddstr(win, "\n\n\nQuando estiver pronto comece a digitar...");
    wattroff(win, COLOR_PAIR(CYAN));

    wrefresh(win);
    let mut character = getch();
    if character == 32 {
        // SPACE
        words[current_word_index].typed = true;
        words[current_word_index].correct = words[current_word_index].is_correct();
        current_word_index += 1;
        current_char_index = 0;
    } else if let Some(pressed_key) = from_code_to_key(character) {
        words[current_word_index].update(&pressed_key, current_char_index);
        current_char_index += 1;
    }

    let start = Instant::now();

    while current_word_index < words.len() {
        wclear(win);

        words.draw(win, current_char_index, current_word_index, actual_window_width);

        let characters_typed = words.count_typed_characters();
        let characters_per_time_spent = &format!(
            "\n\n{} letras / {:?}",
            characters_typed,
            std::time::Duration::new(start.elapsed().as_secs(), 0)
        );

        wattron(win, COLOR_PAIR(CYAN));
        waddstr(win, characters_per_time_spent);
        wattroff(win, COLOR_PAIR(CYAN));

        wrefresh(win);

        character = getch();
        if character == 127 {
            let mut word = &mut words[current_word_index];
            word.typed = false;
            word.correct = false;
            if current_char_index > 0 {
                current_char_index -= 1;
            } else if current_word_index > 0 {
                current_word_index -= 1;
                current_char_index = words[current_word_index].written_chars.len();
            }
            word = &mut words[current_word_index];
            if current_char_index >= word.chars.len() {
                word.extra_written_chars.pop();
            } else {
                word.chars[current_char_index].typed = false;
            }
            word.written_chars.pop();
        } else if character != 127 {
            if character == 32 {
                // SPACE
                words[current_word_index].typed = true;
                words[current_word_index].correct = words[current_word_index].is_correct();
                current_word_index += 1;
                current_char_index = 0;
            } else if let Some(pressed_key) = from_code_to_key(character) {
                words[current_word_index].update(&pressed_key, current_char_index);
                current_char_index += 1;
            }
        }
    }

    delwin(win);
    endwin();

    std::process::Command::new("stty sane");

    let typing_results = compute_typing_results(start, words);
    let font = FIGfont::standard().expect("Não deu pra iniciar o FIGFont pra mostrar os PPM");
    let formatted_wpm = (typing_results.wpm * 100f32).floor() / 100f32;
    let figure = font
        .convert(&format!("{} ppm", formatted_wpm))
        .expect("Não deu pra pegar a figurinha das PPM");

    println!("{}", figure);
    let formatted_acc = (typing_results.accuracy * 100f32 * 100f32).floor() / 100f32;
    println!(
        "{}% ACC      :      Letras escritas: {} => ✓ {} | ✕ {}",
        formatted_acc,
        typing_results.characters_typed,
        typing_results.correct_characters,
        typing_results.incorrect_characters,
    );
    let raw_formatted_wpm = (typing_results.raw_wpm * 100f32).floor() / 100f32;
    println!(
        "{} puro ppm  em {:?}",
        raw_formatted_wpm,
        std::time::Duration::new(typing_results.time_elapsed.as_secs(), 0)
    );
}
