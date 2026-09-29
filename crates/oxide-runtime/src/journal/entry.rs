use chrono::{ DateTime, Local };


/* 
  - Запись журнала событий.

  ? Зачем тут деление на `JournalEntryCategory` и `JournalEntryLevel`?
  ! Это нужно для разделения понятия "критичность" и "тип" записи.

    например: ошибка в рантайме `lua_vm` это:
    - Error       - это тип
    - Information - это критичость

    потому-что для рантайма глобально это не критично, 
    он перезапустит `lua_vm`` или корректно завершится,
    _но_ это не отменяет того что внутри `lua_vm` во время выполнения была именно ошибка.
*/
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct JournalEntry {
  timestamp: DateTime<Local>,
  message:   String,
  category:  JournalEntryCategory,
  level:     JournalEntryLevel  
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum JournalEntryLevel {
  Information,
  Warning,
  Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum JournalEntryCategory {
  Trace,
  Debug,
  Error,
}

impl JournalEntry {
  pub fn new(
    timestamp: DateTime<Local>,
    message:   String,
    category:  JournalEntryCategory,
    level:     JournalEntryLevel 
  ) -> Self {
    Self {
      timestamp,
      message,
      category,
      level 
    }
  }

  pub fn now(
    category: JournalEntryCategory,
    level:    JournalEntryLevel,
    message:  String,
  ) -> Self {
    let timestamp = Local::now();

    Self::new(timestamp, message, category, level)
  }

  pub fn message(&self) -> &str {
    &self.message
  }

  pub fn timestamp(&self) -> DateTime<Local> {
    self.timestamp
  }

  pub fn category(&self) -> JournalEntryCategory {
    self.category
  }

  pub fn level(&self) -> JournalEntryLevel {
    self.level
  }
}

