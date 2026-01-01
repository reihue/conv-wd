use conv_wd::{Directory, util::Error};

fn main() -> Result<(), Error> {
    // Create a persistent directory for output files wich is ignored by git
    // and which is cleaned up on each run.
    let outdir = Directory::cargo_examples_subdir("output")
        .keep()
        .clean()
        .with_gitignore();

    // Create files inside the directory to demonstrate the `write` methods.
    // the files are created with timestamps in thei names, so their names
    // will be different on each run.
    let timestamp = chrono::offset::Local::now()
        .format("%Y-%m-%d_%H-%M-%S")
        .to_string();
    outdir.write_bytes(
        format!("testfile_from_bytes_{timestamp}.txt"),
        "This is a test file inside the output directory.\n",
    )?;
    outdir.write_string(
        format!("testfile_from_string_{timestamp}.txt"),
        "This is another test file inside the output directory.\n",
    )?;
    // Create structured data (JSON and TOML) files inside the directory.
    let data = answer::the();
    outdir.write_string("answer_string.json", &data)?;
    outdir.write_json("answer_json_1.json", &data)?;
    outdir.write_json("answer_json_2", &data)?;
    outdir.write_json("answer_json_3.txt", &data)?;
    outdir.write_toml("answer_toml_1.toml", &data)?;
    outdir.write_toml("answer_toml_2", &data)?;
    outdir.write_toml("answer_toml_3.txt", &data)?;

    Ok(())
}

/// Module with example Struct for structured data files.
mod answer {
    /// An example Struct that can be serialized to JSON.
    /// This is to demonstrate how to write structured data to files using `Directory`.
    #[derive(serde::Serialize)]
    pub struct Answer {
        answer: u32,
        description: String,
        remarks: Remarks,
    }

    /// From implementation to convert Answer to String (JSON format).
    /// This demonstrates how `write_string` can be used to write structured data.
    /// Note: In production code, this is only recommended if the data type is
    /// only ever serialized to one specific format.
    impl From<&Answer> for String {
        fn from(data: &Answer) -> Self {
            serde_json::to_string_pretty(data).unwrap()
        }
    }

    /// A nested Struct to demonstrate more complex data structures.
    #[derive(serde::Serialize)]
    struct Remarks {
        remarks: String,
    }

    /// Constructor for the example Answer struct to be used in the main function.
    pub fn the() -> Answer {
        Answer {
            answer: 42,
            description: "The answer to life, the universe and everything.".to_string(),
            remarks: Remarks {
                remarks: "You didn't specify the question.".to_string(),
            },
        }
    }
}
