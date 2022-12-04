struct SectionRange {
    pub start: i32,
    pub end: i32,
}

impl SectionRange {
    pub fn new(text: &str) -> Self {
        let mut split = text.split('-');
        Self {
            start: split.next().unwrap().parse().unwrap(),
            end: split.next().unwrap().parse().unwrap(),
        }
    }

    pub fn contains(&self, other: &SectionRange) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn overlaps_with(&self, other: &SectionRange) -> bool {
        !(self.end < other.start || other.end < self.start)
    }
}

fn main() {
    let lines = aoc::read_lines("input.txt").expect("Couldn't open input.txt for reading");

    let mut num_matching_pairs = 0;
    let mut num_overlaps = 0;

    for line in lines {
        match line {
            Ok(text) => {
                let ranges: Vec<SectionRange> = text.split(',').map(SectionRange::new).collect();

                if ranges[0].contains(&ranges[1]) || ranges[1].contains(&ranges[0]) {
                    num_matching_pairs += 1;
                }

                if ranges[0].overlaps_with(&ranges[1]) {
                    num_overlaps += 1;
                }
            }
            Err(_) => break,
        }
    }

    println!("Number of matching pairs: {}", num_matching_pairs);
    println!("Number of overlapping pairs: {}", num_overlaps);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlaps_with_works() {
        assert!(SectionRange{ start: 1, end: 2 }.overlaps_with(&SectionRange{ start: 2, end: 4 }));
        assert!(SectionRange{ start: 2, end: 4 }.overlaps_with(&SectionRange{ start: 1, end: 2 }));
        assert!(SectionRange{ start: 2, end: 2 }.overlaps_with(&SectionRange{ start: 2, end: 2 }));
        assert!(SectionRange{ start: 2, end: 10 }.overlaps_with(&SectionRange{ start: 3, end: 4 }));
        assert!(SectionRange{ start: 3, end: 4 }.overlaps_with(&SectionRange{ start: 2, end: 10 }));

        assert!(!SectionRange{ start: 1, end: 2 }.overlaps_with(&SectionRange{ start: 3, end: 4 }));
        assert!(!SectionRange{ start: 3, end: 4 }.overlaps_with(&SectionRange{ start: 1, end: 2 }));
        assert!(!SectionRange{ start: 2, end: 2 }.overlaps_with(&SectionRange{ start: 3, end: 3 }));
    }
}
