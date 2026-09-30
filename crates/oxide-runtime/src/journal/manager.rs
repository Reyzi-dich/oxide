use std::collections::HashMap;

use super::{ JournalId, Journal, JournalError };


/* 
  ? Что контролирует эта константа? 
  ! Контролирует вместимость журнала по умолчанию.

  ? Чему равна еденица измерения?
  ! Еденица равна одной записи(одному `JournalEntry`).
*/
const DEFAULT_JOURNAL_CAPACITY: usize = 1024;

const STARTING_JOURNAL_ID: u64 = 1;

/*
  ? Почему используется u64, а не AtomicU64?
  ! Потому что менеджер _не_ должен быть разделённым между потоками,
    а следовательно — _не_ нуждается в атомарности операции инкремента.
*/
pub struct JournalManager {
  journals: HashMap<JournalId, Journal>,
  next_id:  u64
}

impl JournalManager {
  pub fn new() -> Self {
    let journals = HashMap::new();
    let next_id  = STARTING_JOURNAL_ID;

    Self {
      journals,
      next_id
    }
  }

  /* 
    ? Почему инкремент `self.next_id` происходит в самом конце?
    ! Для транзакционности: если создание Journal вернёт ошибку через `?`, 
      счётчик ID не сдвинется и состояние менеджера останется валидным.
  */
  pub fn register(
    &mut self, 
    name:     impl Into<String>,
    capacity: Option<usize>
  ) -> Result<JournalId, JournalError> {
    let current_id = JournalId::new(self.next_id);

    let capacity = capacity.unwrap_or(DEFAULT_JOURNAL_CAPACITY);

    let journal = Journal::new(name, capacity)?;

    /*
      ? Зачем здесь проверка результата insert?
      ! Внутренний инвариант: next_id строго монотонен, и ключ гарантированно 
        отсутствует в HashMap. Возврат `Some` означает разрушение целостности 
        состояния рантайма (Internal Invariant Violation).
    */
    if self.journals.insert(current_id, journal).is_some() {
      unreachable!("CRITICAL: JournalId duplicate detected in JournalManager. State corrupted.");
    }

    self.next_id += 1;

    Ok(current_id)
  }
}

impl Default for JournalManager {
  fn default() -> Self {
    Self::new()
  }
}
