use std::vec::Vec;

pub fn get_range(range:&str) -> Vec<u32>{
    let start_and_end = range.split('-')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    
    return start_and_end;
}

pub fn is_range_overlapping_range(range1:&Vec<u32>, range2:&Vec<u32>) -> bool {
    let range1_start = range1.get(0).unwrap();
    let range1_end = range1.get(1).unwrap();
    let range2_start = range2.get(0).unwrap();
    let range2_end = range2.get(1).unwrap();

    let range1_is_overlapping_range2: bool = ((range1_start <= range2_start) & (range1_end >= range2_start)) | ((range1_end >= range2_end) & (range1_start <= range2_end));
    let range2_is_overlapping_range1: bool = ((range2_start <= range1_start) & (range2_end >= range1_start)) | ((range2_end >= range1_end) & (range2_start <= range1_end));
    
    let one_range_contains_the_other: bool = range1_is_overlapping_range2 | range2_is_overlapping_range1;
    one_range_contains_the_other
}