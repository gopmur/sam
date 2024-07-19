use super::*;

#[test]
fn test_validate_name_01() {
    assert_eq!(Branch::validate_name("feature/RCT-2341_something"), true);
}

#[test]
fn test_validate_name_02() {
    assert_eq!(Branch::validate_name("hotfix/RCT-2341_something"), true);
}

#[test]
fn test_validate_name_03() {
    assert_eq!(Branch::validate_name("feature/something"), false);
}

#[test]
fn test_validate_name_04() {
    assert_eq!(Branch::validate_name("hotfix/something"), false);
}

#[test]
fn test_validate_name_05() {
    assert_eq!(Branch::validate_name("feat/RCT-2341_something"), false);
}

#[test]
fn test_validate_name_06() {
    assert_eq!(Branch::validate_name("fix/RCT-2341_something"), false);
}

#[test]
fn test_validate_name_07() {
    assert_eq!(Branch::validate_name("fix/something_RCT-2341"), false);
}

#[test]
fn test_validate_name_08() {
    assert_eq!(Branch::validate_name("feature/something_RCT-2341"), false);
}

#[test]
fn test_validate_name_09() {
    assert_eq!(Branch::validate_name("feature//RCT-1234_something"), false);
}

#[test]
fn test_validate_name_10() {
    assert_eq!(
        Branch::validate_name("feature/RCT-something_something"),
        false
    );
}

#[test]
fn test_validate_name_11() {
    assert_eq!(Branch::validate_name("feature/RCT-1234something"), false);
}

#[test]
fn test_validate_name_12() {
    assert_eq!(Branch::validate_name("featureRCT-1234_something"), false);
}

#[test]
fn test_validate_name_13() {
    assert_eq!(Branch::validate_name("master"), true);
}

#[test]
fn test_validate_name_14() {
    assert_eq!(Branch::validate_name("main"), true);
}

#[test]
fn test_validate_name_15() {
    assert_eq!(Branch::validate_name("develop"), true);
}

#[test]
fn test_validate_name_16() {
    assert_eq!(Branch::validate_name("dfdevelop"), false);
}
