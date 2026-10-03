use miette::SourceSpan;

use super::LuauError;


impl LuauError {
  /*
    - Вычисляет точный байтовый диапазон `SourceSpan` внутри исходного кода файла
      по целевому номеру строки.
    
    * `miette` требует указания абсолютного смещения в байтах `offset` и длины
      выделяемого фрагмента `length` для корректной отрисовки рамки исходного кода
      и маркеров под окном терминала.
  */
  pub fn into_miette_span(&self, source_code: &str) -> SourceSpan {
    // -- если номер строки передан как 0, подсвечиваем весь файл
    if self.line == 0 {
      return SourceSpan::new(0.into(), source_code.len());
    }

    let mut current_offset = 0;

    for (index, line) in source_code.lines().enumerate() {
      let current_line_num = index + 1;

      if current_line_num == self.line {
        /* 
          - Длина строки должна быть не менее 1 байта (даже если строка пустая),
            чтобы miette смог поставить визуальный указатель
        */
        let line_len = line.len().max(1);

        return SourceSpan::new(current_offset.into(), line_len);
      }

      // -- Учитываем длину текущей строки и 1 байт символа переноса строки `\n`
      current_offset += line.len() + 1;
    }

    // -- Если указанный номер строки выходит за пределы файла, возвращаем полный спан
    SourceSpan::new(0.into(), source_code.len())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::path::PathBuf;

  fn create_dummy_error(line: usize) -> LuauError {
    LuauError {
      file_path: PathBuf::from("test.luau"),
      line,
      message: "dummy error message".to_string(),
      stack_trace: None,
    }
  }

  #[test]
  fn test_into_miette_span_first_line() {
    let source = "local x = 10\nlocal y = 20\nreturn x + y";
    let error = create_dummy_error(1);

    let span = error.into_miette_span(source);

    // "local x = 10" -> offset: 0, length: 12
    assert_eq!(span.offset(), 0);
    assert_eq!(span.len(), 12);
  }

  #[test]
  fn test_into_miette_span_middle_line() {
    let source = "local a = 1\nlocal b = \nlocal c = 3";
    // Строка 1: "local a = 1\n" -> 11 байт + 1 (\n) = 12 байт смещения
    // Строка 2: "local b = " -> 10 байт
    let error = create_dummy_error(2);

    let span = error.into_miette_span(source);

    assert_eq!(span.offset(), 12);
    assert_eq!(span.len(), 10);
  }

  #[test]
  fn test_into_miette_span_empty_line_returns_minimum_length_of_one() {
    let source = "local a = 1\n\nlocal b = 2";
    // Строка 2 — пустая. Длина должна быть принудительно 1 (max(1)), чтобы miette смог нарисовать маркер.
    let error = create_dummy_error(2);

    let span = error.into_miette_span(source);

    assert_eq!(span.offset(), 12);
    assert_eq!(span.len(), 1);
  }

  #[test]
  fn test_into_miette_span_out_of_bounds_line_fallback_to_full_source() {
    let source = "local x = 1";
    let error = create_dummy_error(99);

    let span = error.into_miette_span(source);

    // Если строка вылетает за пределы файла, подсвечиваем весь файл целиком
    assert_eq!(span.offset(), 0);
    assert_eq!(span.len(), source.len());
  }

  #[test]
  fn test_into_miette_span_zero_line_fallback_to_full_source() {
    let source = "local x = 1\nlocal y = 2";
    let error = create_dummy_error(0);

    let span = error.into_miette_span(source);

    assert_eq!(span.offset(), 0);
    assert_eq!(span.len(), source.len());
  }
}
