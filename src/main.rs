use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

enum Format {
    Naive,
    Advanced(Params),
}

#[derive(Debug)]
struct Params {
    max_width: usize,
}

fn main() {
    // let text = std::env::args().nth(1).expect("Usage: <binary> TEXT");
    let file = std::env::args().nth(1).expect("Usage: <binary> TEXT");
    let file = File::open(file).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    let formatted = format_k(contents, Format::Naive);
    println!("{}", formatted);
}

fn format_k(text: String, format: Format) -> String {
    match format {
        Format::Naive => naive_format(text),
        Format::Advanced(_) => unimplemented!(),
    }
}

fn naive_format(text: String) -> String {
    let mut out = String::from("");

    let mut level = 0;

    for c in text.chars() {
        if c == '(' {
            out.push(c);
            level += 1;
            out.push('\n');
            // (0..level).for_each(|_| out.push('\t'));
            (0..level).for_each(|_| out.push_str("    "));

            continue;
        } else if c == ')' {
            level -= 1;
            out.push('\n');
            (0..level).for_each(|_| out.push_str("    "));
        }

        out.push(c);
    }

    out
}

#[test]
fn smol_naive() {
    let text = "(KApply(),)".to_string();
    let formatted = format_k(text, Format::Naive);

    let expected = "(
    KApply(
        
    ),
)"
    .to_string();

    assert_eq!(expected, formatted);
}

#[test]
fn format() {
    let text = "(KApply(label=KLabel(name='#Equals', params=(KSort(name='Bool'), KSort(name='GeneratedTopCell'))), args=(KToken(token='true', sort=KSort(name='Bool')), KApply(label=KLabel(name='foundry_success', params=()), args=(KVariable(name='STATUSCODE_FINAL', sort=KSort(name='StatusCode')), KApply(label=KLabel(name='#lookup(_,_)_EVM-TYPES_Int_Map_Int', params=()), args=(KVariable(name='CHEATCODE_STORAGE_FINAL', sort=KSort(name='Map')), KToken(token='46308022326495007027972728677917914892729792999299745830475596687180801507328', sort=KSort(name='Int')))), KVariable(name='ISREVERTEXPECTED_FINAL', sort=KSort(name='Bool')), KVariable(name='ISOPCODEEXPECTED_FINAL', sort=KSort(name='Bool')), KVariable(name='RECORDEVENT_FINAL', sort=KSort(name='Bool')), KVariable(name='ISEVENTEXPECTED_FINAL', sort=KSort(name='Bool')))))),)".to_string();
    let formatted = format_k(text, Format::Advanced(Params { max_width: 100 }));

    let expected = "(
    KApply(
        label=KLabel(
            name='#Equals', params=(KSort(name='Bool'), KSort(name='GeneratedTopCell'))
        ), 
        args=(
            KToken(token='true', sort=KSort(name='Bool')), 
            KApply(
                label=KLabel(
                    name='foundry_success', params=()
                ), 
                args=(
                    KVariable(
                        name='STATUSCODE_FINAL', sort=KSort(name='StatusCode'))
                    , 
                    KApply(
                        label=KLabel(
                            name='#lookup(_,_)_EVM-TYPES_Int_Map_Int', params=()
                        ), 
                        args=(
                            KVariable(name='CHEATCODE_STORAGE_FINAL', sort=KSort(name='Map')), 
                            KToken(
                                token='46308022326495007027972728677917914892729792999299745830475596687180801507328', 
                                sort=KSort(name='Int')
                            )
                        )
                    ), 
                    KVariable(name='ISREVERTEXPECTED_FINAL', sort=KSort(name='Bool')), 
                    KVariable(name='ISOPCODEEXPECTED_FINAL', sort=KSort(name='Bool')), 
                    KVariable(name='RECORDEVENT_FINAL', sort=KSort(name='Bool')), 
                    KVariable(name='ISEVENTEXPECTED_FINAL', sort=KSort(name='Bool'))
                ))
        )
    ),
)"
    .to_string();

    assert_eq!(formatted, expected);
}
