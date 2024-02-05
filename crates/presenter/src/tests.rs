use super::*;

#[test]
fn format_german_numbers() {
    assert_eq!(Lng::De.format_number(0.6), "0,6");
    assert_eq!(Lng::De.format_number(6.0), "6");
    assert_eq!(Lng::De.format_number(6.01), "6,01");

    assert_eq!(Lng::De.format_number_with_precision(6.0, 3), "6,000");
    assert_eq!(Lng::De.format_number_with_precision(0.6, 3), "0,600");
}

#[test]
fn format_german_numbers_with_thousands_separator() {
    assert_eq!(
        Lng::De.format_number_with_thousands_seperator(6_000.0),
        "6.000"
    );

    // TODO: bug or feature?
    // assert_eq!(Lng::De.format_number_with_thousands_seperator(6_000.1), "6.000,1");
}

#[test]
fn format_english_numbers() {
    assert_eq!(Lng::En.format_number(0.6), "0.6");
    assert_eq!(Lng::En.format_number(6.0), "6");
    assert_eq!(Lng::En.format_number(6.01), "6.01");
}

#[test]
fn format_english_numbers_with_thousands_separator() {
    assert_eq!(
        Lng::En.format_number_with_thousands_seperator(6_000.0),
        "6,000"
    );

    // TODO: bug or feature?
    // assert_eq!(Lng::En.format_number_with_thousands_seperator(6_000.01), "6,000.01");
}
