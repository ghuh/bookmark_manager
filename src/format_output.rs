use core::iter;

pub struct FormatOutput {
    url_max: usize,
    desc_max: usize,
    lines: Vec<Line>,
}

struct Line {
    url: String,
    url_len: usize,
    description: String,
    desc_len: usize,
    tags: String,
}

impl FormatOutput {
    pub fn new() -> Self {
        Self {
            url_max: 0,
            desc_max: 0,
            lines: Vec::new(),
        }
    }

    pub fn add_line(
        &mut self,
        url: String,
        url_len: usize,
        description: String,
        desc_len: usize,
        tags: &Vec<&str>,
    ) {
        if url_len > self.url_max {
            self.url_max = url_len;
        }

        if desc_len > self.desc_max {
            self.desc_max = desc_len;
        }

        self.lines.push(
            Line {
                url,
                url_len,
                description,
                desc_len,
                tags: tags.join(" | "),
            }
        );
    }

    pub fn print(&self) {
        for line in &self.lines {
            // Can't use println formatting width because gets messed up by colored lines
            println!(
                "{}{} {}{} {}",
                line.url,
                generate_padding(line.url_len, self.url_max),
                line.description,
                generate_padding(line.desc_len, self.desc_max),
                line.tags,
            );
        }
    }
}

// Inspiration: https://docs.rs/crate/tabwriter/1.2.1/source/src/lib.rs
fn generate_padding(current_len: usize, pad_to: usize) -> String {
    iter::repeat(' ').take(pad_to - current_len).collect()
}