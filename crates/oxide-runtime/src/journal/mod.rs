/*
  Одна запись в журнале
*/
pub mod entry;

/*
  Один журнал  
*/
pub mod journal;

/*
  Управление всеми журналами
*/
pub mod manager;

/*
  Форматирует запись в другой формат, например, цветной текст или json
*/
pub mod formatter;

/*
  Выводит форматированный вывод, наприрмер, в терминал или файл
*/
pub mod sink;


/*
  Pub re-exports
*/
pub use entry::{ JournalEntry, JournalEntryCategory, JournalEntryLevel };
pub use journal::{ Journal, JournalError };
