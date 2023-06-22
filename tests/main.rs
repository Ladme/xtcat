// Released under MIT License.
// Copyright (c) 2022-2023 Ladislav Bartos

#[cfg(test)]
mod pass_tests {
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;

    use assert_cmd::Command;
    use tempfile::NamedTempFile;

    #[test]
    fn concatenate() {
        let output = NamedTempFile::new().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("xtcat")
            .unwrap()
            .args(["-ftests/test_files/split1.xtc tests/test_files/split2.xtc tests/test_files/split3.xtc tests/test_files/split4.xtc tests/test_files/split5.xtc", &output_arg, "--overwrite"])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/joined.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn silent() {
        let output = NamedTempFile::new().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("xtcat")
            .unwrap()
            .args(["-ftests/test_files/split1.xtc tests/test_files/split2.xtc tests/test_files/split3.xtc tests/test_files/split4.xtc tests/test_files/split5.xtc", &output_arg, "--overwrite", "--silent"])
            .assert()
            .success()
            .stdout("");

        assert!(file_diff::diff(
            "tests/test_files/joined.xtc",
            output.path().to_str().unwrap()
        ));
    }

    #[test]
    fn backup() {
        let mut dummy = File::create("tests/real_temporary.xtc").unwrap();
        write!(&mut dummy, "Some input that will be checked.\n").unwrap();
        let output_arg = format!("-otests/real_temporary.xtc");

        Command::cargo_bin("xtcat")
            .unwrap()
            .args(
                ["-ftests/test_files/split1.xtc tests/test_files/split2.xtc tests/test_files/split3.xtc tests/test_files/split4.xtc tests/test_files/split5.xtc", 
                &output_arg])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/joined.xtc",
            "tests/real_temporary.xtc"
        ));

        let backed_up_content = fs::read_to_string("tests/#real_temporary.xtc.1#").unwrap();
        assert_eq!(backed_up_content, "Some input that will be checked.\n");

        // clean the workspace
        fs::remove_file("tests/real_temporary.xtc").unwrap();
        fs::remove_file("tests/#real_temporary.xtc.1#").unwrap();
    }

    #[test]
    fn overwrite() {
        let mut dummy = File::create("tests/real_temporary2.xtc").unwrap();
        write!(&mut dummy, "Some input that will not be checked.\n").unwrap();
        let output_arg = format!("-otests/real_temporary2.xtc");

        Command::cargo_bin("xtcat")
            .unwrap()
            .args(
                ["-ftests/test_files/split1.xtc tests/test_files/split2.xtc tests/test_files/split3.xtc tests/test_files/split4.xtc tests/test_files/split5.xtc", &output_arg, "--overwrite"])
            .assert()
            .success();

        assert!(file_diff::diff(
            "tests/test_files/joined.xtc",
            "tests/real_temporary2.xtc"
        ));

        // check that the back up was not created
        assert!(!Path::new("tests/#real_temporary2.xtc.1#").exists());

        // clean the workspace
        fs::remove_file("tests/real_temporary2.xtc").unwrap();
    }
}

#[cfg(test)]
mod fail_tests {
    use assert_cmd::Command;
    use tempfile::NamedTempFile;

    #[test]
    fn file_does_not_exist() {
        let output = NamedTempFile::new().unwrap();
        let output_arg = format!("-o{}", output.path().display());

        Command::cargo_bin("xtcat")
            .unwrap()
            .args(["-ftests/test_files/split1.xtc tests/test_files/split0.xtc tests/test_files/split3.xtc tests/test_files/split4.xtc tests/test_files/split5.xtc", &output_arg, "--overwrite"])
            .assert()
            .failure()
            .code(2);
    }

    #[test]
    fn output_unreachable() {
        let output = "unreachable/very_unreachable/output.xtc";
        let output_arg = format!("-o{}", output);

        Command::cargo_bin("xtcat")
            .unwrap()
            .args(["-ftests/test_files/split1.xtc tests/test_files/split2.xtc tests/test_files/split3.xtc tests/test_files/split4.xtc tests/test_files/split5.xtc", &output_arg, "--overwrite"])
            .assert()
            .failure()
            .code(1);
    }
}
