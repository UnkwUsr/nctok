use crate::entry::Entry;
use crate::ConfigArgs;
use indexmap::IndexMap;
use std::io;

#[derive(clap::Args)]
pub struct ParserConfig {
    #[rustfmt::skip]
    #[arg(long, default_value = " ", help = "Delimiter between element number value and path")]
    pub number_delimiter: char,
    #[arg(long, default_value = "/", help = "Separator in element path")]
    pub path_separator: char,
}

type EntryPath = Vec<String>;

impl Entry {
    fn add<CmpFn>(&mut self, path: EntryPath, name: String, value: usize, cmp_fn: &CmpFn)
    where
        CmpFn: Fn(&Entry, &Entry) -> std::cmp::Ordering,
    {
        self.size += value;

        let childs = self
            .children
            .get_or_insert(IndexMap::<String, Entry>::new());

        if path.is_empty() {
            childs.insert(
                name,
                Entry {
                    size: value,
                    children: None,
                },
            );
            // TODO: future optimization (probably): sort once after everything added
            childs.sort_unstable_by(|_ak, av, _bk, bv| cmp_fn(av, bv));
            return;
        }

        let key = path[0].clone();
        let child = childs.entry(key).or_insert(Entry {
            size: 0,
            children: None,
        });

        child.add(path[1..].to_vec(), name, value, cmp_fn);
        childs.sort_unstable_by(|_ak, av, _bk, bv| cmp_fn(av, bv));
    }
}

pub fn parse_stdin(config: ConfigArgs) -> Entry {
    let mut root = Entry {
        size: 0,
        children: None,
    };

    let cmp_fn = if config.reverse {
        |a: &Entry, b: &Entry| b.size.cmp(&a.size).reverse()
    } else {
        |a: &Entry, b: &Entry| b.size.cmp(&a.size)
    };

    io::stdin().lines().for_each(|x| {
        if let Some((number, suffix)) = x.unwrap().split_once(config.parser.number_delimiter) {
            let number: usize = number.parse().unwrap();

            let mut path = suffix.split(config.parser.path_separator);
            let name = path.next_back().unwrap().to_string();
            let parent_path: EntryPath = path.map(str::to_string).collect();

            root.add(parent_path, name, number, &cmp_fn);
        }
    });

    root
}
