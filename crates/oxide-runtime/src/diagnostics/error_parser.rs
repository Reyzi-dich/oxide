use std::path::PathBuf;

use crate::runtime::error::InitError;

#[derive(Debug, PartialEq, Eq)]
pub struct LuauDiagnostic {
  pub stack_trace: Option<String>,
  pub file_path:   PathBuf,
  pub message:     String,
  pub line:        usize,
}

impl LuauDiagnostic {
  fn parse(raw_message: &str) -> Result<Self, InitError> {
    // -- Отрезаем начальный префикс `[string "`
    let text_after_prefix = raw_message
      .strip_prefix("[string \"")
      .expect("строка ошибки Luau должна начинаться с '[string \"'");

    // -- Ищем закрывающую кавычку и двоеточие `"]:`
    let closing_quote_position = text_after_prefix
      .find("\"]:")
      .expect("не найден закрывающий маркер пути '\"]:'");

    let extracted_path_str = &text_after_prefix[..closing_quote_position];

    // -- Переходим к сегменту после `"]:` (там лежат номер строки и сообщение)
    let text_after_path = &text_after_prefix[closing_quote_position + 3..];

    // -- Ищем двоеточие, отделяющее номер строки от текста ошибки
    let line_number_delimiter_position = text_after_path
      .find(':')
      .expect("не найдено двоеточие после номера строки");

    let line_number_str = &text_after_path[..line_number_delimiter_position];
    
    let line_number = line_number_str
      .trim()
      .parse::<usize>()
      .expect("номер строки в ошибке Luau не является числом");

    // -- Забираем остаток текста ошибки
    let full_remaining_text = &text_after_path[line_number_delimiter_position + 1..];

    // -- Разделяем сообщение и трейсбек, если маркер `stack traceback:` присутствует
    let (extracted_message, extracted_stack_trace) = match full_remaining_text.split_once("\nstack traceback:") {
      Some((message_part, traceback_part)) => {
        let clean_message = message_part.trim().to_string();
        let clean_traceback = traceback_part.trim().to_string();
        
        (clean_message, Some(clean_traceback))
      }
      None => {
        // -- Если трейсбека нет (например, при синтаксической ошибке), берём первую строку
        let first_line = full_remaining_text
          .lines()
          .next()
          .unwrap_or(full_remaining_text)
          .trim()
          .to_string();

        (first_line, None)
      }
    };

    Ok(LuauDiagnostic {
      file_path:   PathBuf::from(extracted_path_str),
      line:        line_number,
      message:     extracted_message,
      stack_trace: extracted_stack_trace,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_valid_syntax_error() {
    let raw_error_input = "[string \"examples/test.luau\"]:15: Incomplete statement: expected assignment or a function call";

    let parsed_error = LuauDiagnostic::parse(raw_error_input).unwrap();

    assert_eq!(parsed_error.file_path, PathBuf::from("examples/test.luau"));
    assert_eq!(parsed_error.line, 15);
    assert_eq!(
      parsed_error.message,
      "Incomplete statement: expected assignment or a function call"
    );
  }

  #[test]
  fn test_parse_runtime_error_preserves_stack_trace() {
    let raw_error_input = "[string \"src/main.luau\"]:42: attempt to perform arithmetic\nstack traceback:\n\t[string \"src/main.luau\"]:42: in function 'calculate'\n\t[string \"src/main.luau\"]:100: in main chunk";

    let parsed_error = LuauDiagnostic::parse(raw_error_input).unwrap();

    assert_eq!(parsed_error.file_path, PathBuf::from("src/main.luau"));
    assert_eq!(parsed_error.line, 42);
    assert_eq!(parsed_error.message, "attempt to perform arithmetic");
    
    assert_eq!(
      parsed_error.stack_trace,
      Some("[string \"src/main.luau\"]:42: in function 'calculate'\n\t[string \"src/main.luau\"]:100: in main chunk".to_string())
    );
  }

  #[test]
  fn test_parse_error_with_spaces_around_line_and_message() {
    let raw_error_input = "[string \"config.luau\"]:  7  :   unexpected symbol near 'end'  ";

    let parsed_error = LuauDiagnostic::parse(raw_error_input).unwrap();

    assert_eq!(parsed_error.file_path, PathBuf::from("config.luau"));
    assert_eq!(parsed_error.line, 7);
    assert_eq!(parsed_error.message, "unexpected symbol near 'end'");
  }
}

