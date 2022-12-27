use std::ops::Range;

use serde::{Serialize, Deserialize};

/// Struct representing a command.
#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

impl Command {
  pub fn set(key: String, value: String) -> Command {
      Command::Set { key, value }
  }

  pub fn remove(key: String) -> Command {
      Command::Remove { key }
  }

}

#[derive(Debug, Clone, Copy)]
pub(crate) struct CommandPos {
    pub(crate) gen: u64,
    pub(crate) pos: u64,
    pub(crate) len: u64,
}

impl From<(u64, Range<u64>)> for CommandPos {
    fn from((gen, range): (u64, Range<u64>)) -> Self {
        CommandPos {
            gen,
            pos: range.start,
            len: range.end - range.start,
        }
    }
}