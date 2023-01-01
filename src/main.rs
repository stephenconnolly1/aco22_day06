
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
                let mut len = find_start_of_packet(&x);
                println!("Packet marker at location: {len}");
                len = find_start_of_message(&x);
                println!("message marker at location: {len}");
            }
        }
    } else {
        println!("Unable to open file");
    }
}

fn find_start_of_packet(stream: &str) -> usize {
    return find_end_of_marker(stream, 4);
}

fn find_start_of_message(stream: &str) -> usize {
    return find_end_of_marker(stream, 14);
}


fn find_end_of_marker(stream: &str, marker_size:usize) -> usize {
    
    'outer: for i in 0..(stream.len()-marker_size) {
        let s = &stream[i..i+marker_size];
        let chars: Vec<_> = s.chars().collect();

        for x in 0..marker_size {
            for y in x+1..marker_size {
                if chars[x] == chars[y] {continue 'outer;}
            }
        }
        // All chars in this segment unique
        return i+marker_size;
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
    //message tests
    #[test]
    fn t5() {
        let s = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(find_start_of_message(&s), 19);
    }
    #[test]
    fn t6() {
        let s = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(find_start_of_message(&s), 23);
    }
    #[test]
    fn t7() {
        let s = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(find_start_of_message(&s), 23);
    }
    #[test]
    fn t8() {
        let s = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(find_start_of_message(&s), 29);
    }
    #[test]
    fn t9() {
        let s = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(find_start_of_message(&s), 26);
    }
}
