use std::collections::VecDeque;

use super::{
  JournalEntry,
  JournalError,
  JournalOperationalError,
  JournalInternalError
};


#[derive(Debug, Error, PartialEq, Eq)]
pub enum JournalError {
  #[error("Journal capacity must be greater than 0")]
  ZeroCapacity,
}

/* 
  * VecDeque - это двусторонний кольцевой буфер с фиксированной вместимостью(`capacity``).

  ? Почему используеться VecDeque вместо стандартного Vec?
  ! 1. Контроль памяти: 
    Обычный Vec растёт бесконечно. В долгом процессе (или при циклическом логе из Luau)
    это приводит к неконтролируемому поеданию RAM вплоть до OOM (Out Of Memory).
  ! 2. Производительность O(1):
    При достижении лимита памяти нам нужно выкидывать самую старую запись из начала.
    В обыкновенном Vec вызов `.remove(0)` требует физического сдвига ВСЕХ элементов в памяти (сложность O(N)).
    В VecDeque удаление из начала (`.pop_front()`) происходит мгновенно (сложность O(1)).

  - Размер журнала указывается при создании, может изменяться по ходу выполнения и контролируется владельцем.

  * Несмотря на возможности Deque, добавлять записи разрешено _только_ в конец (`push_back`), 
    чтобы гарантировать строгую хронологическую последовательность событий.
  
  ? Почему бы не использовать VecQueue?
  ! Такого типа просто не существует в стандартной библиотеке Rust (`std::collections`).

*/
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Journal {
  capacity: usize,
  entries:  VecDeque<JournalEntry>
}

impl Journal {
  /*
    - Создание пустого журнала.
  */
  pub fn new(capacity: usize) -> Result<Self, JournalError> {
    if capacity == 0 {
      return Err(ZeroCapacity);
    }

    let entries = VecDeque::with_capacity(capacity);
    
    Ok(Self {
      capacity,
      entries
    })
  }

  /*
    - Изменение вместимости журнала на лету.
  */
  pub fn set_capacity(&mut self, new_capacity: usize) -> Result<(), JournalError> {
    if new_capacity == 0 {
      return Err(JournalError::ZeroCapacity);
    }

    self.capacity = new_capacity;

    while self.entries.len() > self.capacity {
      self.entries.pop_front();
    }

    // -- Освобождаем неиспользуемую память в куче
    self.entries.shrink_to_fit();

    Ok(())
  }

  /* 
    - Добавление записи.
    * Если достигнут лимит вместимости(`capacity`),
      то самая старая запись автоматически удаляется.
      Это то, ради чего и используеться именно `VecDeque`
  */
  pub fn push(&mut self, entry: JournalEntry) {
    if self.entries.len() >= self.capacity {
      // -- Удаляем старую запись
      self.entries.pop_front(); 
    }
    
    self.entries.push_back(entry);
  }

  pub fn entries(&self) -> &VecDeque<JournalEntry> {
    &self.entries
  }

  /*
    - Забрать все записи удалив их из журнала.
    * Отдаёт все записи журнала и полностью очищает сам журнал.
      Используеться, например, для батч метода вывод в терминал
  */
  pub fn drain(&mut self) -> Vec<JournalEntry> {
    self.entries.drain(..).collect()
  }

  pub fn capacity(&self) -> usize {
    self.capacity
  }

  pub fn len(&self) -> usize {
    self.entries.len()
  }

  pub fn is_empty(&self) -> bool {
    self.entries.is_empty()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::journal::{JournalEntryCategory, JournalEntryLevel};

  fn make_entry(msg: &str) -> JournalEntry {
    JournalEntry::now(
      JournalEntryCategory::Debug,
      JournalEntryLevel::Information,
      msg.to_string(),
    )
  }

  #[test]
  fn test_ring_buffer_overflow() {
    let mut journal = Journal::new(2).unwrap();
    
    journal.push(make_entry("first"));
    journal.push(make_entry("second"));
    journal.push(make_entry("third"));

    assert_eq!(journal.entries().len(), 2);
    assert_eq!(journal.entries()[0].message(), "second");
    assert_eq!(journal.entries()[1].message(), "third");
  }

  #[test]
  fn test_zero_capacity_error() {
    assert_eq!(Journal::new(0), Err(JournalError::ZeroCapacity));
  }

  #[test]
  fn test_shrink_capacity() {
    let mut journal = Journal::new(5).unwrap();

    for i in 0..5 {
      journal.push(make_entry(&format!("msg {i}")));
    }

    journal.set_capacity(2).unwrap();
    assert_eq!(journal.entries().len(), 2);
    assert_eq!(journal.entries()[0].message(), "msg 3");
    assert_eq!(journal.entries()[1].message(), "msg 4");
  }
}
