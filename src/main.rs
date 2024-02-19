use clap::Parser;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    path::PathBuf,
};

/* Do not modify this */
// CLI args parser
#[derive(Parser, Debug)]
pub struct Args {
    // Csv file read path
    #[arg(short, long)]
    pub read_path: PathBuf,
    // Sub command for handling data in csv file
    #[clap(subcommand)]
    pub command: Command,
}

/* Do not modify this */
// Command and args prser
#[derive(Parser, Debug)]
pub enum Command {
    // Display entire file
    Display,
    // Modify a row/field
    Replace {
        #[clap(short, long)]
        row: usize,

        #[clap(short, long)]
        col: usize,

        // the new item to put into csv file
        #[clap(short, long)]
        data: String,
    },
}


#[allow(dead_code)]
#[derive(Debug, Default)]
struct CSVFile {
    data: Vec<Vec<String>>,
    rows: usize,
    cols: usize,
}

impl CSVFile {
  pub fn replace(&mut self, row: usize, col: usize, data: String) {
    self.data[row][col] = data;
  }
}

pub trait CSVFileReader {
    fn read(&mut self, file_path: PathBuf) -> Result<(), Error>;
}

impl CSVFileReader for CSVFile {
    fn read(&mut self, file_path: PathBuf) -> Result<(), Error> {
        let file = File::open(file_path)?;
        let buff = BufReader::new(file);

        let mut data: Vec<Vec<String>> = Vec::new();

        for (index, line) in buff.lines().enumerate() {
          let cols = line.split(",").collect::<Vec<String>>();
          data.push(cols);
        }

        self.data = data;
        self.rows = data.len();
        self.cols = data[0].len();

        Ok(())
    }
}

fn main() {
    // Reading CLI args
    let args = Args::parse();

    // create CSVFile instance and read file into it
    let mut csv = CSVFile::default();
    let _ = csv.read(args.read_path);

    // match and execute command
    match args.command {
        Command::Display => println!("{:?}", csv),
        Command::Replace { row, col, data } => {
          csv.replace(row, col, data);
        },
    }
}
