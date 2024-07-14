#!/bin/bash
set -e
cargo install toml-cli
current_version=$(toml get -r Cargo.toml 'package.version')

git tag "v$current_version"

git checkout dev
git merge origin/main


new_version=$(echo $current_version | awk -F. '/[0-9]+\./{$NF++;print}' OFS=.)
echo "Current version is $current_version, new version will be ${new_version}"
mkdir -p target/tomls/
toml set Cargo.toml package.version $new_version > target/tomls/a.tom
toml set target/tomls/a.tom dependencies.rust-regex-dsl_derive.version $new_version > target/tomls/b.tom
toml set target/tomls/b.tom dependencies.rust-regex-dsl-creator.version $new_version > Cargo.toml
toml set rust-regex-dsl_derive/Cargo.toml package.version $new_version > target/tomls/c.tom
mv target/tomls/c.tom rust-regex-dsl_derive/Cargo.toml
toml set rust-regex-dsl-creator/Cargo.toml package.version $new_version > target/tomls/c.tom
mv target/tomls/c.tom rust-regex-dsl-creator/Cargo.toml

rm -rf target/tomls/

git add ./Cargo.toml ./rust-regex-dsl-creator/Cargo.toml ./rust-regex-dsl_derive/Cargo.toml
git commit -m $new_version
