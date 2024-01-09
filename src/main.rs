use semver::{Version, VersionReq};

fn valid_official(version: Version) -> bool {
    let min = Version::parse("1.0.0-beta.9").unwrap();
    let max = Version::parse("2.0.0").unwrap();
    min <= version && version < max
}

fn valid_custom(version: Version) -> bool {
    let req = VersionReq::parse(">=1.0.0-beta.9, <2.0.0").unwrap();
    VersionReq::matches(&req, &version)
}

enum Valid {
    Official,
    Custom
}

impl Valid {
    fn check(&self, version: Version) -> bool {
        match self {
            Valid::Official => valid_official(version),
            Valid::Custom => valid_custom(version)
        }
    }
}

fn check_versions(valid: Valid) {
    let v1_0_0_beta_3 = Version::parse("1.0.0-beta.3").unwrap();
    let v1_0_0_beta_9 = Version::parse("1.0.0-beta.9").unwrap();
    let v1_0_0 = Version::parse("1.0.0").unwrap();
    let v1_1_0_beta_3 = Version::parse("1.1.0-beta.3").unwrap();
    let v1_1_0 = Version::parse("1.1.0").unwrap();
    let v2_0_0 = Version::parse("2.0.0").unwrap();

    let valid_v1_0_0_beta_3 = valid.check(v1_0_0_beta_3);
    let valid_v1_0_0_beta_9 = valid.check(v1_0_0_beta_9);
    let valid_v1_0_0 = valid.check(v1_0_0);
    let valid_v1_1_0_beta_3 = valid.check(v1_1_0_beta_3);
    let valid_v1_1_0 = valid.check(v1_1_0);
    let valid_v2_0_0 = valid.check(v2_0_0);

    println!("v1.0.0-beta.3 | {valid_v1_0_0_beta_3}");
    println!("v1.0.0-beta.9 | {valid_v1_0_0_beta_9}");
    println!("v1.0.0        | {valid_v1_0_0}");
    println!("v1.1.0-beta.3 | {valid_v1_1_0_beta_3}");
    println!("v1.1.0        | {valid_v1_1_0}");
    println!("v2.0.0        | {valid_v2_0_0}");
}

fn main() {
    println!(" ");
    println!("Official validation");
    println!("-----------------------");
    check_versions(Valid::Official);

    println!(" ");
    println!("Custom validation");
    println!("-----------------------");
    check_versions(Valid::Custom);

    println!(" ");
}
