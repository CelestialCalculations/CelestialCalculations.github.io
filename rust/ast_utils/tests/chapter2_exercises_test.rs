use ast_utils::ast_math::{floor, is_close};
use ast_utils::distance::{DistanceConverter, DistanceUnit};
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
