use std::io::BufRead;

pub fn get_words_from_file() -> Vec<String> {
    let file_name = "src/words.txt";
    let mut lines = Vec::new();

    let file = std::fs::File::open(file_name).expect("Failed to open file");

    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        lines.push(line.expect("Failed to read line"));
    }

    lines
}
