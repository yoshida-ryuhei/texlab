use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(tree_sitter_latex::language()).unwrap();

    let code = include_str!("../examples/texlab.tex");

    c.bench_function("init", |b| {
        b.iter(|| {
            let mut parser = tree_sitter::Parser::new();
            parser.set_language(tree_sitter_latex::language()).unwrap();
            parser
        });
    });

    c.bench_function("full", |b| {
        b.iter_with_large_drop(|| parser.parse(black_box(code), None).unwrap());
    });

    let tree = parser.parse(code, None).unwrap();
    c.bench_function("traverse", |b| {
        b.iter(|| {
            let mut cursor = tree.walk();
            loop {
                cursor.node();

                if !cursor.goto_first_child() {
                    if !cursor.goto_next_sibling() {
                        loop {
                            if !cursor.goto_parent() {
                                return;
                            }

                            if cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                }
            }
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
