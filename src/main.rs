
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(x) = line {
                let len = find_start_of_packet(&x);
                println!("Packet marker at location: {len}");
            }
        }
    } else {
        println!("Unable to open file");
    }
}
fn find_start_of_packet(stream: &str) -> usize {
    
    for i in 0..(stream.len()-4) {
        let s = &stream[i..i+4];
        let mut chars = s.chars();
        let c1 = chars.next().unwrap();
        let c2 = chars.next().unwrap();
        let c3 = chars.next().unwrap();
        let c4 = chars.next().unwrap();

        if c1 == c2 {continue;}
        if c1 == c3 {continue;}
        if c1 == c4 {continue;}

        if c2 == c3 {continue;}
        if c2 == c4 {continue;}

        if c3 == c4 {continue;}

        // All 4 chars in this quartet unique
        return i+4;
    }
    println! ("No packet start sequence found");
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let s = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(find_start_of_packet(&s), 5);
    }

    #[test]
    fn t2() {
        let s = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(find_start_of_packet(&s), 6);
    }
    #[test]
    fn t3() {
        let s = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(find_start_of_packet(&s), 10);
    }
    #[test]
    fn t4() {
        let s = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(find_start_of_packet(&s), 11);
    }
}
