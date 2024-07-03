use super::*;

mod format {
    use super::*;

    #[test]
    fn german_numbers() {
        let lng = Lng::De;
        assert_eq!(lng.format_number(0.6), "0,6");
        assert_eq!(lng.format_number(6.0), "6");
        assert_eq!(lng.format_number(6.01), "6,01");
        assert_eq!(lng.format_number(6_000), "6.000");
        assert_eq!(lng.format_number(6_000.0), "6.000");
        assert_eq!(lng.format_number(6_000.1), "6.000,1");
        assert_eq!(lng.format_number(6_000.123_456_789), "6.000,123456789");
    }

    #[test]
    fn english_numbers() {
        let lng = Lng::En;
        assert_eq!(lng.format_number(0.6), "0.6");
        assert_eq!(lng.format_number(6.0), "6");
        assert_eq!(lng.format_number(6.01), "6.01");
        assert_eq!(lng.format_number(6_000), "6,000");
        assert_eq!(lng.format_number(6_000.0), "6,000");
    }

    #[test]
    fn german_numbers_with_fixed_precision() {
        let lng = Lng::De;
        assert_eq!(lng.format_number_with_fixed_precision(6.0, 3), "6,000");
        assert_eq!(lng.format_number_with_fixed_precision(0.6, 3), "0,600");
        assert_eq!(lng.format_number_with_fixed_precision(6_000.1, 0), "6.000");
        assert_eq!(
            lng.format_number_with_fixed_precision(6_000.0, 3),
            "6.000,000"
        );
    }

    #[test]
    fn german_numbers_with_thousands_separator() {
        let lng = Lng::De;
        assert_eq!(lng.format_number_with_fixed_precision(6_000.0, 0), "6.000");
        assert_eq!(
            lng.format_number_with_fixed_precision(6_000.1, 1),
            "6.000,1"
        );
        assert_eq!(lng.format_number_with_fixed_precision(6_000.1, 0), "6.000");
        assert_eq!(lng.format_number(6_000.1), "6.000,1");
    }

    #[test]
    fn count_value_with_thousands_separator() {
        assert_eq!(Lng::De.format_value(&Value::count(6_000)), "6.000");
    }

    #[test]
    fn english_numbers_with_thousands_separator() {
        let lng = Lng::En;
        assert_eq!(lng.format_number(6_000.0), "6,000");
        assert_eq!(lng.format_number(6_000.01), "6,000.01");
    }

    #[test]
    fn f64_as_german_string() {
        assert_eq!(
            Lng::De.format_f64(23_222_221_231.766_6, None),
            "23.222.221.231,7666"
        );
        assert_eq!(Lng::De.format_f64(23_222_221_231.0, None), "23.222.221.231");
        assert_eq!(Lng::De.format_f64(2.0, None), "2");
    }

    #[test]
    fn f64_as_english_string() {
        assert_eq!(
            Lng::En.format_f64(23_222_221_231.766_6, None),
            "23,222,221,231.7666"
        );
        assert_eq!(Lng::En.format_f64(23_222_221_231.0, None), "23,222,221,231");
        assert_eq!(Lng::En.format_f64(2.0, None), "2");
    }
}

mod parse {
    use super::*;

    #[test]
    fn german_string_as_f64() {
        let result = Lng::De.parse_str_as_f64("1.100.100,23");
        assert_eq!(result, Ok(1_100_100.23));
    }

    #[test]
    fn german_string_as_f64_trailing_space() {
        let result = Lng::De.parse_str_as_f64("1.100.100,23 ");
        assert_eq!(result, Ok(1_100_100.23));
    }

    #[test]
    fn german_string_as_f64_leading_space() {
        let result = Lng::De.parse_str_as_f64(" 1.100.100,23");
        assert_eq!(result, Ok(1_100_100.23));
    }
}
