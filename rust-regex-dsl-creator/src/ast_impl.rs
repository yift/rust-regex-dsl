use std::fmt::Display;

use regex_syntax::ast::{
    Alternation, Assertion, AssertionKind, Ast, ClassBracketed, ClassPerl, ClassPerlKind, ClassSet,
    ClassSetBinaryOp, ClassSetBinaryOpKind, ClassSetItem, ClassUnicode, ClassUnicodeKind, Concat,
    Flag, FlagsItemKind, Group, GroupKind, Literal, Repetition, RepetitionKind, RepetitionRange,
    SetFlags,
};

use crate::printer::Printer;

pub trait Builder {
    fn print_basic_ast(&self, printer: &mut Printer) -> bool;
    fn print_ast(&self, printer: &mut Printer)
    where
        Self: ToString,
    {
        if !self.print_basic_ast(printer) {
            let regex = self.to_string();
            printer.println(format!("regex(\"{}\")", escape_string(&regex)));
        }
    }
}

impl Builder for Ast {
    fn print_basic_ast(&self, printer: &mut Printer) -> bool {
        match self {
            Ast::Concat(cnt) => cnt.print_basic_ast(printer),
            Ast::Literal(lit) => lit.print_basic_ast(printer),
            Ast::Dot(_) => {
                printer.println("any_character".to_string());
                true
            }
            Ast::ClassUnicode(unicode) => unicode.print_basic_ast(printer),
            Ast::Alternation(alts) => alts.print_basic_ast(printer),
            Ast::Assertion(assertion) => assertion.print_basic_ast(printer),
            Ast::ClassPerl(cls) => cls.print_basic_ast(printer),
            Ast::Repetition(rep) => rep.print_basic_ast(printer),
            Ast::Group(group) => group.print_basic_ast(printer),
            Ast::Flags(flags) => flags.print_basic_ast(printer),
            Ast::ClassBracketed(brc) => brc.print_basic_ast(printer),
            Ast::Empty(_) => true,
        }
    }
}
impl Builder for ClassSet {
    fn print_basic_ast(&self, printer: &mut Printer) -> bool {
        match &self {
            ClassSet::Item(item) => item.print_basic_ast(printer),
            ClassSet::BinaryOp(op) => op.print_basic_ast(printer),
        }
    }
}
impl Builder for ClassBracketed {
    fn print_basic_ast(&self, printer: &mut Printer) -> bool {
        let mut indented = Printer::new();

        if !self.kind.print_basic_ast(&mut indented) {
            return false;
        }
        let func = if !self.negated {
            "any_of"
        } else {
            "not_any_of"
        };
        printer.println(format!("{} {{", func));
        printer.append(&indented);
        printer.println("}".to_string());
        true
    }
}
impl Builder for ClassSetItem {
    fn print_basic_ast(&self, printer: &mut Printer) -> bool {
        match self {
            ClassSetItem::Literal(lit) => {
                printer.println(format!("'{}'", lit.c));
                true
            }
            ClassSetItem::Range(range) => {
                printer.println(format!("from: '{}', to: '{}'", range.start.c, range.end.c));
                true
            }
            ClassSetItem::Unicode(cls) => cls.print_basic_ast(printer),
            ClassSetItem::Perl(cls) => cls.print_basic_ast(printer),
            ClassSetItem::Union(u) => {
                for i in &u.items {
                    if !i.print_basic_ast(printer) {
                        return false;
                    }
                }
                true
            }
            ClassSetItem::Bracketed(c) => {
                if c.negated {
                    false
                } else {
                    c.kind.print_basic_ast(printer)
                }
            }
            _ => false,
        }
    }
}
impl Builder for ClassSetBinaryOp {
    fn print_basic_ast(&self, printer: &mut Printer) -> bool {
        if !self.lhs.print_basic_ast(printer) {
            return false;
        }
        let mut indented = Printer::new();
        if !self.rhs.print_basic_ast(&mut indented) {
            return false;
        }
        let name = match self.kind {
            ClassSetBinaryOpKind::Difference => "subtract",
            ClassSetBinaryOpKind::Intersection => "intersect",
            ClassSetBinaryOpKind::SymmetricDifference => "xor",
        };
        printer.println(format!("{} {{", name));
        printer.append(&indented);
        printer.println("}".to_string());
        true
    }
}

impl Builder for SetFlags {
    fn print_basic_ast(&self, printer: &mut Printer) -> bool {
        let mut indented = Printer::new();
        let mut apply = true;
        for flag in &self.flags.items {
            match flag.kind {
                FlagsItemKind::Negation => apply = false,
                FlagsItemKind::Flag(flag) => {
                    if !(apply, flag).print_basic_ast(&mut indented) {
                        return false;
                    }
                }
            }
        }
        printer.println("apply {".to_string());
        printer.append(&indented);
        printer.println("}".to_string());
        true
    }
}
impl Builder for (bool, Flag) {
    fn print_basic_ast(&self, printer: &mut Printer) -> bool {
        let name = match self.1 {
            Flag::CaseInsensitive => Some("case_insensitive"),
            Flag::MultiLine => Some("multi_line"),
            Flag::DotMatchesNewLine => Some("allow_dot"),
            Flag::CRLF => Some("allow_dot"),
            _ => None,
        };
        let prefix = if self.0 { "+" } else { "-" };
        match name {
            Some(name) => {
                printer.println(format!("{}{}", prefix, name));
                true
            }
            None => false,
        }
    }
}
impl Builder for Group {
    fn print_basic_ast(&self, printer: &mut Printer) -> bool {
        match &self.kind {
            GroupKind::CaptureIndex(_) => {
                let mut indented = Printer::new();
                self.ast.print_ast(&mut indented);
                printer.println("group {".to_string());
                printer.append(&indented);
                printer.println("}".to_string());
            }
            GroupKind::CaptureName {
                starts_with_p: _,
                name,
            } => {
                let mut indented = Printer::new();
                indented.println(format!("name: {}", name.name));
                self.ast.print_ast(&mut indented);
                printer.println("group {".to_string());
                printer.append(&indented);
                printer.println("}".to_string());
            }
            GroupKind::NonCapturing(flags) => {
                if flags.items.is_empty() {
                    self.ast.print_ast(printer);
                } else {
                    let mut indented = Printer::new();
                    let mut apply = true;
                    for flag in &flags.items {
                        match flag.kind {
                            FlagsItemKind::Negation => apply = false,
                            FlagsItemKind::Flag(flag) => {
                                if !(apply, flag).print_basic_ast(&mut indented) {
                                    return false;
                                }
                            }
                        }
                    }
                    self.ast.print_ast(&mut indented);
                    printer.println("apply {".to_string());
                    printer.append(&indented);
                    printer.println("}".to_string());
                }
            }
        };
        true
    }
}
impl Builder for Repetition {
    fn print_basic_ast(&self, printer: &mut Printer) -> bool {
        let mut indented = Printer::new();
        let prefix = match &self.op.kind {
            RepetitionKind::ZeroOrMore => "maybe_repeat {",
            RepetitionKind::ZeroOrOne => "maybe {",
            RepetitionKind::OneOrMore => "repeat {",
            RepetitionKind::Range(range) => {
                match range {
                    RepetitionRange::Exactly(n) => indented.println(format!("exactly: {}", n)),
                    RepetitionRange::AtLeast(n) => indented.println(format!("at_least: {}", n)),
                    RepetitionRange::Bounded(at_least, at_most) => {
                        indented.println(format!("at_least: {}, at_most: {}", at_least, at_most))
                    }
                }
                "times {"
            }
        };
        if !self.greedy {
            indented.println("lazy".to_string())
        }
        self.ast.print_ast(&mut indented);
        printer.println(prefix.to_string());
        printer.append(&indented);
        printer.println("}".to_string());
        true
    }
}

impl Builder for ClassPerl {
    fn print_basic_ast(&self, printer: &mut Printer) -> bool {
        let name = match self.kind {
            ClassPerlKind::Digit => "digit",
            ClassPerlKind::Space => "white_space",
            ClassPerlKind::Word => "word_character",
        };
        let prefix = if self.negated { "not_" } else { "" };
        printer.println(format!("{}{}", prefix, name));
        true
    }
}
impl Builder for Assertion {
    fn print_basic_ast(&self, printer: &mut Printer) -> bool {
        let name = match self.kind {
            AssertionKind::StartLine => "beginning_of_line",
            AssertionKind::EndLine => "end_of_line",
            AssertionKind::StartText => "beginning_of_input",
            AssertionKind::EndText => "end_of_input",
            AssertionKind::WordBoundary => "word_boundary",
            AssertionKind::NotWordBoundary => "not_word_boundary",
            _ => return false,
        };
        printer.println(name.to_string());
        true
    }
}
impl Builder for Alternation {
    fn print_basic_ast(&self, printer: &mut Printer) -> bool {
        let mut indented = Printer::new();
        for ast in &self.asts {
            ast.print_ast(&mut indented)
        }
        printer.println("any {".to_string());
        printer.append(&indented);
        printer.println("}".to_string());
        true
    }
}
impl Builder for ClassUnicode {
    fn print_basic_ast(&self, printer: &mut Printer) -> bool {
        let letter = if self.negated { "~" } else { "#" };
        let name = match &self.kind {
            ClassUnicodeKind::Named(name) => name.to_string(),
            ClassUnicodeKind::OneLetter(l) => l.to_string(),
            _ => return false,
        };
        printer.println(format!("{}{}", letter, name));
        true
    }
}
impl Builder for Literal {
    fn print_basic_ast(&self, printer: &mut Printer) -> bool {
        printer.println(format!("eq(\"{}\")", escape_char(&self.c)));
        true
    }
}
impl Builder for Concat {
    fn print_basic_ast(&self, printer: &mut Printer) -> bool {
        let in_concat = combine_concat(&self.asts);
        match in_concat.len() {
            0 => true,
            1 => in_concat[0].print_basic_ast(printer),
            _ => {
                printer.println("concat {".to_string());
                let mut indented = Printer::new();
                for element in in_concat {
                    element.print_ast(&mut indented)
                }
                printer.append(&indented);
                printer.println("}".to_string());
                true
            }
        }
    }
}

enum InConcat<'h> {
    Str(String),
    Char(char),
    Ast(&'h Ast),
}
fn combine_concat(asts: &[Ast]) -> Vec<InConcat<'_>> {
    let mut ret = Vec::new();
    let mut current_str = String::new();
    let mut chr = None;
    for ast in asts {
        if let Ast::Literal(c) = ast {
            if let Some(c1) = chr {
                chr = None;
                current_str.push(c1);
                current_str.push(c.c);
            } else if current_str.is_empty() {
                chr = Some(c.c)
            } else {
                current_str.push(c.c);
            }
        } else {
            if !current_str.is_empty() {
                ret.push(InConcat::Str(current_str));
                current_str = String::new();
            }
            if let Some(c) = chr {
                ret.push(InConcat::Char(c));
                chr = None;
            }
            ret.push(InConcat::Ast(ast))
        }
    }
    if !current_str.is_empty() {
        ret.push(InConcat::Str(current_str));
    }
    if let Some(c) = chr {
        ret.push(InConcat::Char(c));
    }

    ret
}

impl Display for InConcat<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InConcat::Ast(ast) => ast.fmt(f),
            InConcat::Char(c) => c.fmt(f),
            InConcat::Str(str) => str.fmt(f),
        }
    }
}

impl Builder for InConcat<'_> {
    fn print_basic_ast(&self, printer: &mut Printer) -> bool {
        match self {
            InConcat::Ast(ast) => ast.print_basic_ast(printer),
            InConcat::Char(chr) => {
                printer.println(format!("eq(\"{}\")", escape_char(chr)));
                true
            }
            InConcat::Str(str) => {
                printer.println(format!("eq(\"{}\")", escape_string(str)));
                true
            }
        }
    }
}
fn escape_string(string: &str) -> String {
    string.chars().map(|c| escape_char(&c)).collect()
}
fn escape_char(chr: &char) -> String {
    match chr {
        '\n' => "\\n".to_string(),
        '\r' => "\\r".to_string(),
        '\t' => "\\t".to_string(),
        '\\' => "\\\\".to_string(),
        '\"' => "\\\"".to_string(),
        _ => format!("{}", chr),
    }
}
