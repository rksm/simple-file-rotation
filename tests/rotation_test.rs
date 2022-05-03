use simple_file_rotation::{FileRotation, Result};
use std::io::Write;
use std::{fs, path::PathBuf};

struct Fixture {
    test_dir: PathBuf,
}

impl Fixture {
    fn new(test_name: &str) -> Self {
        let dir = format!("{}-{test_name}", env!("CARGO_CRATE_NAME"));
        let test_dir = std::env::temp_dir().join(dir);
        fs::create_dir(&test_dir).expect("create test dir");
        Self { test_dir }
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        if let Err(err) = fs::remove_dir_all(&self.test_dir) {
            eprintln!("Unable to clean up test dir {:?}: {err}", self.test_dir);
        }
    }
}

#[test]
fn no_file_to_rotate() -> Result<()> {
    let fixture = Fixture::new("no_file_to_rotate");

    let dir = fs::read_dir(&fixture.test_dir)?;
    assert_eq!(dir.count(), 0, "test directory is not empty");

    FileRotation::new(fixture.test_dir.join("logfile.log")).rotate()?;

    let dir = fs::read_dir(&fixture.test_dir)?;
    assert_eq!(
        dir.count(),
        0,
        "test directory is not empty even though there are no files to rotate"
    );

    Ok(())
}

#[test]
fn single_file_to_rotate() -> Result<()> {
    let fixture = Fixture::new("single_file_to_rotate");

    let file = fixture.test_dir.join("logfile.log");
    let content = "testing\ntesting\n";

    {
        let mut file = match fs::File::create(&file) {
            Err(err) => panic!("unable to open test file: {err}"),
            Ok(file) => file,
        };
        write!(file, "{content}")?;
    }

    FileRotation::new(&file).rotate()?;

    let mut dir = fs::read_dir(&fixture.test_dir)?;
    let entry = match dir.next().unwrap() {
        Err(err) => panic!("unable to read dir with rotated file: {err}"),
        Ok(entry) => entry,
    };

    assert!(dir.next().is_none(), "more than one file?");

    let file = fixture.test_dir.join("logfile.1.log");
    assert_eq!(entry.path(), file);
    assert_eq!(fs::read_to_string(file)?, content);

    Ok(())
}

#[test]
fn two_files_to_rotate() -> Result<()> {
    let fixture = Fixture::new("two_files_to_rotate");

    let content1 = "content1\n";
    let content2 = "content2\n";
    fs::write(fixture.test_dir.join("logfile.log"), content1)?;
    fs::write(fixture.test_dir.join("logfile.1.log"), content2)?;

    FileRotation::new(fixture.test_dir.join("logfile.log")).rotate()?;

    let mut entries = fs::read_dir(&fixture.test_dir)?
        .map(|ea| ea.unwrap().file_name().to_string_lossy().to_string())
        .collect::<Vec<_>>();
    entries.sort();

    assert_eq!(
        entries,
        vec!["logfile.1.log".to_string(), "logfile.2.log".to_string()]
    );

    assert_eq!(
        fs::read_to_string(fixture.test_dir.join("logfile.2.log"))?,
        content2
    );
    assert_eq!(
        fs::read_to_string(fixture.test_dir.join("logfile.1.log"))?,
        content1
    );

    Ok(())
}

#[test]
fn dont_create_more_than_allowed_rotated_files() -> Result<()> {
    let fixture = Fixture::new("dont_create_more_than_allowed_rotated_files");

    let content1 = "content1\n";
    let content2 = "content2\n";
    let content3 = "content3\n";
    let content4 = "content4\n";
    fs::write(fixture.test_dir.join("logfile.log"), content1)?;
    fs::write(fixture.test_dir.join("logfile.1.log"), content2)?;
    fs::write(fixture.test_dir.join("logfile.2.log"), content3)?;
    fs::write(fixture.test_dir.join("logfile.3.log"), content4)?;

    FileRotation::new(fixture.test_dir.join("logfile.log"))
        .max_old_files(2)
        .rotate()?;

    let mut entries = fs::read_dir(&fixture.test_dir)?
        .map(|ea| ea.unwrap().file_name().to_string_lossy().to_string())
        .collect::<Vec<_>>();
    entries.sort();

    assert_eq!(
        entries,
        vec!["logfile.1.log".to_string(), "logfile.2.log".to_string()]
    );

    assert_eq!(
        fs::read_to_string(fixture.test_dir.join("logfile.2.log"))?,
        content2
    );
    assert_eq!(
        fs::read_to_string(fixture.test_dir.join("logfile.1.log"))?,
        content1
    );

    Ok(())
}
