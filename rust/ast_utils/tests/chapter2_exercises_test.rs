use ast_utils::angular::{self, AngularConverter, AngularUnit};
use ast_utils::ast_math::{floor, is_close};
use ast_utils::distance::{DistanceConverter, DistanceUnit};
use ast_utils::temperature::{TemperatureConverter, TemperatureUnit};
use ast_utils::traits::*;

// TODO: type of result is too complicated - simplify the trait
fn assert_distance<'a>(
    expected_val: f64,
    res: Result<Box<dyn HasConvertableUnit<Unit = DistanceUnit>>, &'a str>,
) {
    assert!(res.is_ok());
    let res = res.unwrap();
    let rounded = floor(res.scalar(), 6);
    assert!(is_close(expected_val, rounded), "res was: {}", rounded);
}

fn assert_approx(expected_val: f64, actual_val: f64) {
    let rounded = floor(actual_val, 6);
    assert!(is_close(expected_val, rounded), "res was: {}", rounded);
}

#[test]
fn test_chapter2_ex1() {
    let conv = DistanceConverter::new(5.0, DistanceUnit::Millimeter);

    assert_distance(0.196_850, conv.convert(DistanceUnit::Inch))
}

#[test]
fn test_chapter2_ex2() {
    let conv = DistanceConverter::new(10.0, DistanceUnit::Inch);

    assert_distance(254.0, conv.convert(DistanceUnit::Millimeter));
}

#[test]
fn test_chapter2_ex3() {
    let conv = DistanceConverter::new(30.0, DistanceUnit::Meter);

    assert_distance(98.425_196, conv.convert(DistanceUnit::Feet))
}

#[test]
fn test_chapter2_ex4() {
    let conv = DistanceConverter::new(25.0, DistanceUnit::Feet);

    assert_distance(7.62, conv.convert(DistanceUnit::Meter));
}

#[test]
fn test_chapter2_ex5() {
    let conv = DistanceConverter::new(100.0, DistanceUnit::Mile);

    assert_distance(160.9344, conv.convert(DistanceUnit::Kilometer));
}

#[test]
fn test_chapter2_ex6() {
    let conv = DistanceConverter::new(88.0, DistanceUnit::Kilometer);

    assert_distance(54.680_664, conv.convert(DistanceUnit::Mile));
}

#[test]
fn test_chapter2_ex7() {
    let u = DistanceConverter::new(12.0, DistanceUnit::LightYear);

    assert_distance(7.0548e13, u.convert(DistanceUnit::Mile)); // book mistake: checked with WolframAlpha
}

#[test]
fn test_chapter2_ex8() {
    let u = DistanceConverter::new(9.3e7, DistanceUnit::Mile);

    assert_distance(1.5e-5, u.convert(DistanceUnit::LightYear));
}

#[test]
fn test_chapter2_ex9() {
    let u = DistanceConverter::new(5.0, DistanceUnit::LightYear);

    assert_distance(1.533, u.convert(DistanceUnit::Parsec));
}

#[test]
fn test_chapter2_ex10() {
    let u = DistanceConverter::new(3.0, DistanceUnit::Parsec);

    assert_distance(9.784_692, u.convert(DistanceUnit::LightYear)); // book error: checked with WolframAlpha
}

#[test]
fn test_chapter2_ex11() {
    let u = DistanceConverter::new(2.0, DistanceUnit::AstronomicalUnit);

    assert_distance(1.8580e8, u.convert(DistanceUnit::Mile));
}

#[test]
fn test_chapter2_ex12() {
    let u = DistanceConverter::new(10_000.0, DistanceUnit::Mile);

    assert_distance(1.07e-4, u.convert(DistanceUnit::AstronomicalUnit));
}

#[test]
fn test_chapter2_ex13() {
    let u = AngularConverter::new(180.0, AngularUnit::DMS);
    let res = u.convert(AngularUnit::Radian);

    assert!(res.is_ok());
    let res = res.unwrap();
    assert_approx(3.141_592, res.scalar());
}

#[test]
fn test_chapter2_ex14() {
    let u = AngularConverter::new(2.5, AngularUnit::Radian);
    let res = u.convert(AngularUnit::DMS);

    assert!(res.is_ok());
    let res = res.unwrap();
    assert_approx(143.239_444, res.scalar());
}

#[test]
fn test_chapter2_ex15() {
    let u = AngularConverter::new(2.0, AngularUnit::HMS);
    let res = u.convert(AngularUnit::DMS);

    assert!(res.is_ok());
    let res = res.unwrap();
    assert_approx(30.0, res.scalar());
}

#[test]
fn test_chapter2_ex16() {
    let u = AngularConverter::new(156.3, AngularUnit::DMS);
    let res = u.convert(AngularUnit::HMS);

    assert!(res.is_ok());
    let res = res.unwrap();
    assert_approx(10.419_722, res.scalar());
}

#[test]
fn test_chapter2_ex17() {
    let u = angular::HMS::new(10, 25, 11);

    assert_approx(10.419_722, u.scalar());
}

#[test]
fn test_chapter2_ex18() {
    let u = angular::HMS::from_decimal(20.352);

    assert_eq!((20, 21, 7), u.to_tuple());
}

#[test]
fn test_chapter2_ex19() {
    let u = angular::DMS::new(13, 4, 10);

    assert_approx(13.069_444, u.scalar());
}

#[test]
fn test_chapter2_ex20() {
    let u = angular::DMS::from_decimal(-0.508_334);

    assert_eq!((-0, 30, 30), u.to_tuple());
}

#[test]
fn test_chapter2_ex21() {
    let u = angular::DMS::new(300, 20, 0);

    assert_approx(300.333_333, u.to_decimal());
}

#[test]
fn test_chapter2_ex22() {
    let u = angular::DMS::from_decimal(10.2958);

    assert_eq!((10, 17, 44), u.to_tuple());
}

#[test]
fn test_chapter2_ex23() {
    let u = TemperatureConverter::new(100.0, TemperatureUnit::Celsius);
    let res = u.convert(TemperatureUnit::Fahrenheit);

    assert_approx(212.0, res.unwrap().scalar());
}

#[test]
fn test_chapter2_ex24() {
    let u = TemperatureConverter::new(32.0, TemperatureUnit::Fahrenheit);
    let res = u.convert(TemperatureUnit::Celsius);

    assert_approx(0.0, res.unwrap().scalar());
}
