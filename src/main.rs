#![feature(try_blocks)]
#![feature(str_split_once)]

use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;

use anyhow::{Context, Result as AnyResult};
use rand::{prelude::*, distributions::WeightedIndex};

fn main() -> AnyResult<()> {
    let fpath = std::env::args_os().nth(1)
        .context("Missing file path argument")?;

    let ngen: u32 = std::env::args().nth(2)
        .unwrap_or("1".into())
        .parse()?;

    let file = std::fs::File::open(fpath)?;
    let file = BufReader::new(file);

    let mut values = vec![];
    let mut weights = vec![];

    for (i, line) in file.lines().enumerate() {
        let r: AnyResult<_> = try {
            let line = line?;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let (w,v) = line.split_once(' ')
                .context("missing weight")?;
            let w: u32 = w.parse()?;

            (w,v.to_string())
        };

        match r {
            Ok((w,v)) => {
                values.push(v);
                weights.push(w);
            }
            Err(e) => {
                eprintln!("error (line {}): {}", i, e);
            }
        }
    }

    let mut dist = WeightedIndex::new(weights.iter())?;

    let mut rng = thread_rng();

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    let mut last = None;
    for _ in 0..ngen {
        let next = if let Some(last) = last {
            dist.update_weights(&[(last, &0)]);
            let next = dist.sample(&mut rng);
            dist.update_weights(&[(last, &weights[last])]);
            next
        } else {
            dist.sample(&mut rng)
        };
        last = Some(next);
        writeln!(stdout, "{}", values[next])?;
    }

    Ok(())
}
