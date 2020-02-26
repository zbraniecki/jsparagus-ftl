use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use std::fs::File;
use std::io;
use std::io::Read;

use jsparagus_ftl::parser::Parser;

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn parser_bench(c: &mut Criterion) {
    let tests = &["simple"];

    c.bench_function_over_inputs(
        "parser_bench",
        move |b, &name| {
            let source = read_file(&format!("./benches/{}.js", name)).unwrap();
            b.iter(|| {
                let parser = Parser::new(&source);
                let _ = parser.parse();
            });
        },
        tests,
    );
}

criterion_group!(benches, parser_bench);
criterion_main!(benches);
