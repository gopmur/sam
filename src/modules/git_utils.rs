struct Branch {
    branch_name: String,
    branch_type: String,
    branch_code: String,
}

impl Branch {
    const VALID_TYPES: [&str; 2] = ["feature", "hotfix"];
    const SPECIAL_NAMES: [&str; 3] = ["develop", "main", "master"];
    fn validate_name(name: &str) -> bool {
        if name.is_empty() {
            return false;
        }
        if Self::SPECIAL_NAMES.contains(&name) {
            return true;
        };
        'outer: for valid_type in Self::VALID_TYPES.iter() {
            if !name.starts_with(&format!("{}/RCT-", valid_type)) {
                continue;
            };
            let branch_code_start = valid_type.len() + "/RCT-".len();

            for (i, c) in name[branch_code_start..].chars().enumerate() {
                if i == 0 && (c < '0' || c > '9') || i == name.len() - branch_code_start - 1 {
                    continue 'outer;
                }
                if c >= '0' && c <= '9' {
                    continue;
                }
                if c == '_' {
                    break;
                }
            }
            return true;
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
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
}

// pub fn get_branch_code() -> String {
//   let branch_name = Command::new("git")
//       .arg("branch")
//       .arg("--show-current")
//       .output()
//       .unwrap()
//       .stdout;
//   let branch_name = String::from_utf8(branch_name).unwrap();
//   for branch_type in supported_branch_types.iter() {
//       if branch_name.starts_with(branch_type) {
//           continue;
//       }
//       panic!("Branch type not supported");
//   }

//   return String::from("");
// }
