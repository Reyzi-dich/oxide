use std::io;
use std::fmt;
use std::path::PathBuf;

use crate::runtime::Runtime;

#[derive(Debug)]
pub struct Engine {
  entry_path: PathBuf,
  options:    EngineOptions,
  runtime:    Runtime,
}

#[derive(Debug, Clone, Default)]
pub struct EngineOptions {
  debug: bool,
  trace: bool
}

#[derive(Debug, thiserror::Error)]
pub enum EngineError {
  #[error("Failed to read entry file {}: {error}", path.display())]
  EntryFileReadFailed { 
    error: io::Error, 
    path:  PathBuf 
  },
}

impl Engine {
  pub fn new(entry_path: PathBuf) -> Self {
    let runtime = Runtime::new();
    let options = EngineOptions::default();

    Engine { 
      entry_path,
      options,
      runtime
    }
  }

  fn read_entry_file_content(&self) -> Result<String, EngineError> {
    std::fs::read_to_string(&self.entry_path)
      .map_err(|error| {
        let path = self.entry_path.clone();

        EngineError::EntryFileReadFailed { path, error }
      })
  }

  pub fn run(&mut self) -> Result<(), EngineError> {
    let entry_file_content = self.read_entry_file_content()?;

    Ok(())
  }
}
