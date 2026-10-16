#[test]
fn integer128_boundaries() {
    for (source, expected) in [
        ("0", 0i128),
        ("-0", 0),
        ("1", 1),
        ("-1", -1),
        ("170141183460469231731687303715884105727", i128::MAX),
        ("-170141183460469231731687303715884105728", i128::MIN),
    ] {
        assert_eq!(serde_json::from_str::<i128>(source).unwrap(), expected);
    }

    for (source, expected) in [
        ("0", 0u128),
        ("1", 1),
        ("340282366920938463463374607431768211455", u128::MAX),
    ] {
        assert_eq!(serde_json::from_str::<u128>(source).unwrap(), expected);
    }

    for source in [
        "170141183460469231731687303715884105728",
        "-170141183460469231731687303715884105729",
        "999999999999999999999999999999999999999999999999999",
        "01",
        "1e",
        "1.0",
        "-",
    ] {
        assert!(serde_json::from_str::<i128>(source).is_err(), "{source}");
    }
    for source in [
        "340282366920938463463374607431768211456",
        "-1",
        "01",
        "1e",
        "1.0",
    ] {
        assert!(serde_json::from_str::<u128>(source).is_err(), "{source}");
    }
}
