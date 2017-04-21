static TEXT: &str = r#"
1 hello world
2 hello
  world
3 hello world
4 hello world
"#;


// static EXPECTATION: &[&str] = [
// "1 hello world",
// "2 hello world",
// "3 hello world",
// "4 hello world",
// ];
// 
#[test]
fn test_split_ical_style() {
    // iter over str and take 2 chars (window)
    //  
    // let lines: Vec<String> = TEXT.split(|[a, b]| a == '\n' && !b.is_whitespace()).collect();
    struct State {
      buffer: Vec<char>,
      last: Option<char>,
    }
    

    let initial_state = State{
        buffer : Vec::with_capacity(100),
        last: None,
      };

      // "1 a\n2 b\n c"
      // "1 a", "2 b\n c"
      // "1 a", "2 bc"

    TEXT.chars().scan(initial_state, |state, &c|{
      if !c.is_whitespace() && state.last == Some('\n') {
        // normal break, this is a new line
        // let 
        // state.buffer.push(c);
        state.last = Some(c);
        return None;
        // return Some(buffer);

      } else if c.is_whitespace() && state.last != Some('\n') {
        // whitespace is part of the content, keep
        state.buffer.push(c);
        state.last = Some(c);
        return None;

      } else if !c.is_whitespace() && c != '\n'  {
        // normal character, keep
        state.buffer.push(c);
        state.last = Some(c);

        return None;

      } else {
        state.last = c;

      }

      

    }).collect();
}


fn split_icalendar(text: &str) -> Vec<String> {
  let mut prev_c = None;
  let mut buf = String::new();
  let mut lines = Vec::new();
  let mut skip_space = false;

  for c in text.chars() {
    if let Some(prev_c) = prev_c {
    match (prev_c, c) {
      ('\n', ' ') => { skip_space = true; },
      ('\n', '\n') => {},
      ('\n', c) => { lines.push(buf.clone()); buf.clear(); },
      (' ', c) => if skip_space { skip_space = false; } else { buf.push(' '); }
      (p, c) => { buf.push(p); },
    }
    }

    prev_c = Some(c);
  }

  if prev_c != Some('\n') {
    buf.push(prev_c.unwrap());
  }
  
  if !buf.is_empty() {
    lines.push(buf);
  }
  
  return lines;
}
