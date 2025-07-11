use super::*;

#[test]
fn test_validate_name_01() {
    assert_eq!(Branch::validate_name("feature/2341_something"), true);
}

#[test]
fn test_validate_name_02() {
    assert_eq!(Branch::validate_name("hotfix/2341_something"), true);
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
    assert_eq!(Branch::validate_name("feat/2341_something"), false);
}

#[test]
fn test_validate_name_06() {
    assert_eq!(Branch::validate_name("fix/2341_something"), false);
}

#[test]
fn test_validate_name_07() {
    assert_eq!(Branch::validate_name("fix/something_2341"), false);
}

#[test]
fn test_validate_name_08() {
    assert_eq!(Branch::validate_name("feature/something_2341"), false);
}

#[test]
fn test_validate_name_09() {
    assert_eq!(Branch::validate_name("feature//1234_something"), false);
}

#[test]
fn test_validate_name_10() {
    assert_eq!(
        Branch::validate_name("feature/something_something"),
        false
    );
}

#[test]
fn test_validate_name_11() {
    assert_eq!(Branch::validate_name("feature/1234something"), false);
}

#[test]
fn test_validate_name_12() {
    assert_eq!(Branch::validate_name("feature1234_something"), false);
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

#[test]
fn test_parse_name_1() -> Result<(), Error> {
    assert_eq!(
        Branch::parse_name("feature/13-fix-build-warnings")?,
        (
            String::from("feature"),
            String::from("13"),
            String::from("fix-build-warnings"),
            false
        )
    );
    Ok(())
}

#[test]
fn test_parse_name_2() {
    assert_eq!(
        Branch::parse_name("master"),
        Ok((
            String::from(""),
            String::from(""),
            String::from("master"),
            true
        ))
    )
}

#[test]
fn test_parse_name_3() {
    assert_eq!(
        Branch::parse_name("feature/1234something"),
        Err(Error::NameFormat)
    );
}

#[test]
fn test_make_commit_name_1() {
    let branch = Branch {
        branch_code: "2222".to_string(),
        branch_title: "something".to_string(),
        branch_type: "feature".to_string(),
        is_special: false,
    };
    assert_eq!(
        branch.make_commit_message(CommitType::Feat, "this is a commit", false),
        "feat(#2222): this is a commit"
    );
}

#[test]
fn test_make_commit_name_2() {
    let branch = Branch {
        branch_code: "".to_string(),
        branch_title: "master".to_string(),
        branch_type: "".to_string(),
        is_special: true,
    };
    assert_eq!(
        branch.make_commit_message(CommitType::Feat, "this is a commit", false),
        "feat: this is a commit"
    );
}

#[test]
fn test_make_commit_name_3() {
    let branch = Branch {
        branch_code: "".to_string(),
        branch_title: "master".to_string(),
        branch_type: "".to_string(),
        is_special: true,
    };
    assert_eq!(
        branch.make_commit_message(CommitType::Feat, "this is a commit", true),
        "feat: this is a commit (run_ci)"
    );
}

#[test]
fn test_make_commit_name_4() {
    let branch = Branch {
        branch_code: "2222".to_string(),
        branch_title: "something".to_string(),
        branch_type: "feature".to_string(),
        is_special: false,
    };
    assert_eq!(
        branch.make_commit_message(CommitType::Feat, "this is a commit", true),
        "feat(#2222): this is a commit (run_ci)"
    );
}
