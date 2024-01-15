use super::*;

#[test]
fn format_units() {
    let value = Percent::new(55.2);
    assert_eq!(format!("{value}"), "55.2%");

    let value = Factor::new(0.552);
    assert_eq!(format!("{value}"), "0.552");
}

#[test]
fn convert_percent_to_factor() {
    let percent = Percent::new(55.2);
    let factor = percent.convert_to::<Factor>();
    assert_eq!(f64::from(factor), 0.552);
}

#[test]
fn convert_kilograms_to_tons() {
    let tons: Tons = Kilograms::new(1_000.0).convert_to();
    assert_eq!(f64::from(tons), 1.0);
}

#[test]
fn convert_mg_per_l_to_kg_per_m3() {
    let kg_per_m3: KilogramsPerQubicmeter = MilligramsPerLiter::new(1_000.0).convert_to();
    assert_eq!(f64::from(kg_per_m3), 1.0);
}

#[test]
fn multiply_kg_per_m3_with_m3() {
    let kg_per_m3 = KilogramsPerQubicmeter(5.0);
    let m3 = Qubicmeters(5.0);
    assert_eq!(m3 * kg_per_m3, Kilograms::new(25.0));
}

#[test]
fn kwh_with_g_per_kwh() {
    let g_per_kwh = GramsPerKilowatthour(5.0);
    let kwh = Kilowatthours(1.0);
    assert_eq!(g_per_kwh * kwh, Grams::new(5.0));
}
