use std::fmt::Display;

#[derive(Default)]
pub struct Printer {
    out: Vec<String>,
}

impl Printer {
    pub fn new() -> Self {
        Printer::default()
    }
    pub fn append(&mut self, indented: &Self) {
        let indent = "  ";
        for line in &indented.out {
            let comma = match line.chars().last() {
                Some(',') | Some('{') => "",
                _ => ",",
            };
            let line = format!("{}{}{}", indent, line, comma);
            self.println(line);
        }
    }
    pub fn println(&mut self, line: String) {
        self.out.push(line);
    }
}

impl Display for Printer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for l in &self.out {
            writeln!(f, "{}", l)?;
        }
        Ok(())
    }
}
