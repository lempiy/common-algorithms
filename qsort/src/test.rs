use super::*;

#[test]
fn test_qsort_numbers() {
    let mut v = vec![
        66, 10, 67, 41, 86, 94, 33, 26, 91, 55, 70, 87, 63, 13, 27, 49, 1, 70, 78, 92, 62, 4, 7,
        73, 77, 85, 39, 17, 63, 52, 94, 94, 62, 15, 21, 42, 19, 59, 80, 50, 80, 30, 12, 69, 12, 55,
        95, 64, 75, 1,
    ];
    qsort(&mut v);
    let sorted = vec![
        1, 1, 4, 7, 10, 12, 12, 13, 15, 17, 19, 21, 26, 27, 30, 33, 39, 41, 42, 49, 50, 52, 55, 55,
        59, 62, 62, 63, 63, 64, 66, 67, 69, 70, 70, 73, 75, 77, 78, 80, 80, 85, 86, 87, 91, 92, 94,
        94, 94, 95,
    ];
    for i in 0..v.len() {
        assert_eq!(v[i], sorted[i]);
    }
}

#[test]
fn test_qsort_strings() {
    let mut v = vec![
        "principle",
        "outfit",
        "seminar",
        "regular",
        "corruption",
        "common",
        "water",
        "endorse",
        "excess",
        "waist",
        "objective",
        "institution",
        "species",
        "minor",
        "sex",
        "draw",
        "beer",
        "leave",
        "fade",
        "drill",
        "eavesdrop",
        "pole",
        "new",
        "sweater",
        "context",
        "necklace",
        "deputy",
        "opponent",
        "official",
        "general",
        "ranch",
        "modernize",
        "pitch",
        "potential",
        "first",
        "applaud",
        "match",
        "pluck",
        "inflation",
        "deteriorate",
        "magnetic",
        "business",
        "glow",
        "waterfall",
        "zero",
        "harmony",
        "production",
        "appoint",
        "bad",
        "notice",
    ];
    qsort(&mut v);
    let sorted = vec![
        "applaud",
        "appoint",
        "bad",
        "beer",
        "business",
        "common",
        "context",
        "corruption",
        "deputy",
        "deteriorate",
        "draw",
        "drill",
        "eavesdrop",
        "endorse",
        "excess",
        "fade",
        "first",
        "general",
        "glow",
        "harmony",
        "inflation",
        "institution",
        "leave",
        "magnetic",
        "match",
        "minor",
        "modernize",
        "necklace",
        "new",
        "notice",
        "objective",
        "official",
        "opponent",
        "outfit",
        "pitch",
        "pluck",
        "pole",
        "potential",
        "principle",
        "production",
        "ranch",
        "regular",
        "seminar",
        "sex",
        "species",
        "sweater",
        "waist",
        "water",
        "waterfall",
        "zero",
    ];
    for i in 0..v.len() {
        assert_eq!(v[i], sorted[i]);
    }
}

#[test]
fn test_qsort_zero() {
    let mut v: Vec<i64> = vec![];
    qsort(&mut v);
    assert_eq!(v.len(), 0);
}

#[test]
fn test_qsort_one() {
    let mut v = vec![42];
    qsort(&mut v);
    assert_eq!(v[0], 42);
}

#[test]
fn test_qsort_two() {
    let mut v = vec![42, 12];
    qsort(&mut v);
    assert_eq!(v[0], 12);
    assert_eq!(v[1], 42);
}

#[test]
fn test_qsort_three() {
    let mut v = vec![99, 12, 42];
    qsort(&mut v);
    assert_eq!(v[0], 12);
    assert_eq!(v[1], 42);
    assert_eq!(v[2], 99);
}
