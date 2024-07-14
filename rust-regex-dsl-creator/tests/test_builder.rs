use itertools::Itertools;
use regex::Regex;
use rust_regex_dsl_creator::ToDsl;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::Result;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use tempdir::TempDir;
use toml::map::Map;
use toml::Table;
use toml::Value;

#[derive(Debug, Deserialize, Serialize, Default)]
struct Test {
    name: String,
    regex: String,
    should_pass: Vec<String>,
    should_fail: Vec<String>,
    groups: Option<Vec<GroupCapture>>,
}
#[derive(Debug, Deserialize, Serialize)]
struct GroupCapture {
    haystack: String,
    expected_groups: Vec<ExpectedGroups>,
}
#[derive(Debug, Deserialize, Serialize)]
struct ExpectedGroups {
    name_or_index: String,
    expected_value: String,
}

struct TestSuite {
    root: PathBuf,
    my_root: PathBuf,
    tests: Vec<Test>,
}
impl TestSuite {
    fn test_all(&self) -> Result<()> {
        self.prepare_code()?;
        for t in &self.tests {
            t.sanity_test();
            t.test(self)?;
        }
        Ok(())
    }

    fn prepare_code(&self) -> Result<()> {
        let dir = &self.root.join("code");
        fs::remove_dir_all(dir).ok();
        fs::create_dir_all(dir)?;
        env::set_current_dir(dir)?;

        let vec = self.tests.iter().map(|t| t.prepare()).join(",\n");

        let ec = Command::new("cargo")
            .arg("init")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .expect("Fail to create project!");
        assert!(ec.success());

        let cargo_toml_file = dir.join("Cargo.toml");
        let cargo_toml = fs::read_to_string(&cargo_toml_file)?;
        let mut cargo_toml: Table = toml::from_str(&cargo_toml).unwrap();
        if let Some(Value::Table(dependencies)) = cargo_toml.get_mut("dependencies") {
            let mut dsl = Map::new();
            let path = Value::String(self.my_root.parent().unwrap().to_str().unwrap().to_string());
            dsl.insert("path".to_string(), path);
            dependencies.insert("rust-regex-dsl".into(), Value::Table(dsl));
        } else {
            panic!("Could not get dependencies");
        }
        let cargo_toml = toml::to_string(&cargo_toml).unwrap();
        fs::write(&cargo_toml_file, cargo_toml)?;

        let main_rs = r#"
use std::{collections::HashMap, env};

use rust_regex_dsl::regex_dsl;
fn main() {

    let regexes = vec![
        <to_replace>
    ].into_iter().collect::<HashMap<_, _>>();
    let args: Vec<_> = env::args().collect();
    let name = args.get(1).unwrap();
    println!("name is: {}", name);
    let regex = regexes.get(name.as_str()).unwrap();
    println!("regular expression is: {}", regex);
    let haystack = args.get(2).unwrap();
    println!("haystack is: {}", haystack);
    let should_pass = args.get(3).unwrap() == "yes";
    println!("should pass: {}", should_pass);
    let pass = regex.is_match(&haystack);
    if should_pass == pass {
        println!("OK");
    } else {
        panic!("Wrong!");
    }
    if should_pass {
        if let Some(group_name) = args.get(4) {
            let expected = args.get(5).unwrap();
            println!("Expecting value: {}", &expected);
            let capture = regex.captures(&haystack).unwrap();
            let value = if let Ok(group_number) = group_name.parse::<usize>() {
                capture[group_number].to_string()
            } else {
                capture[group_name.as_str()].to_string()
            };
            println!("Capture value: {}", &value);
            if value != *expected {
                panic!("Wrong!");
            }
        }
    }
}
        
        "#
        .replace("<to_replace>", &vec);
        let main_rs_file = dir.join("src").join("main.rs");
        fs::write(main_rs_file, main_rs)?;

        let status = Command::new("cargo")
            .arg("build")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .expect("Fail to create project!");
        assert!(status.success());
        Ok(())
    }
}
impl Test {
    fn sanity_test(&self) {
        println!("Sanity testing: {}", self.name);
        let regex = Regex::new(&self.regex).unwrap();
        for pass in &self.should_pass {
            println!("\t V {}...", pass);
            assert!(regex.is_match(pass));
        }
        for fail in &self.should_fail {
            println!("\t X {}...", fail);
            assert!(!regex.is_match(fail));
        }
        println!("OK");
    }
    fn prepare(&self) -> String {
        let dsl = self.regex.to_dsl().unwrap();
        println!(
            "For {} which is {} the DSL would look like:\n{}",
            self.name, self.regex, dsl
        );
        format!("(\"{}\", regex_dsl! {{\n{}\n}})", self.name, dsl)
    }

    fn test(&self, suite: &TestSuite) -> Result<()> {
        println!("Testing: {}", self.name,);

        let command = suite
            .root
            .join("code")
            .join("target")
            .join("debug")
            .join("code");
        for pass in &self.should_pass {
            let status = Command::new(&command)
                .args([&self.name, pass, "yes"])
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .unwrap();
            assert!(status.success());
        }
        for fail in &self.should_fail {
            let status = Command::new(&command)
                .args([&self.name, fail, "no"])
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .unwrap();
            assert!(status.success());
        }
        if let Some(groups) = &self.groups {
            for group in groups {
                for expected_group in &group.expected_groups {
                    let status = Command::new(&command)
                        .args([
                            &self.name,
                            &group.haystack,
                            "yes",
                            &expected_group.name_or_index,
                            &expected_group.expected_value,
                        ])
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .status()
                        .unwrap();
                    assert!(status.success());
                }
            }
        }
        println!("OK");
        Ok(())
    }
}
#[test]
fn test_use_cases() -> Result<()> {
    let test_cases_dir = env::current_dir()?.join("tests").join("test_cases");
    let test_cases = fs::read_dir(test_cases_dir)?
        .map(|f| f.unwrap().path())
        .map(|f| fs::read_to_string(f).unwrap())
        .map(|f| toml::from_str(&f).unwrap())
        .collect();

    let root = TempDir::new("creator-tests")?;
    let suite = TestSuite {
        root: root.path().to_path_buf(),
        my_root: env::current_dir()?,
        tests: test_cases,
    };

    suite.test_all()?;

    Ok(())
}
