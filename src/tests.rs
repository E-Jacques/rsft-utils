pub mod test_clean_or_create_dir {
    use std::{fs, path::Path};

    use crate::common::{clean_or_create_dir, file_or_dir_exists};

    #[test]
    fn test_create_missing_directories() {
        let path = Path::new("tests").join("rsc").join("missing_directory");
        let _ = clean_or_create_dir(path.clone());

        assert!(file_or_dir_exists(path.clone()));

        fs::remove_dir(path).expect("should be able to delete test directory")
    }

    #[test]
    fn test_clean_directory() {
        let directory_path = Path::new("tests").join("rsc").join("dir_to_clean");
        fs::create_dir_all(directory_path.clone()).expect("should be able to create directory");

        let files_path = vec!["file_1", "file_2"];
        files_path
            .into_iter()
            .map(|filename| directory_path.clone().join(String::from(filename)))
            .for_each(|filepath| {
                fs::File::create(filepath).expect("should be able to create file");
            });

        assert!(file_or_dir_exists(directory_path.clone().join("file_1")));
        assert!(file_or_dir_exists(directory_path.clone().join("file_2")));

        let _ = clean_or_create_dir(directory_path.clone());

        assert!(!file_or_dir_exists(directory_path.clone().join("file_1")));
        assert!(!file_or_dir_exists(directory_path.clone().join("file_2")));
    }
}
