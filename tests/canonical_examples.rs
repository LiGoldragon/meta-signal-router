use dotos::{DotosEncode, DotosSource};
use meta_signal_router::{z2VVKk, z2VZMR};

#[test]
fn canonical_dotos_examples_are_exact_root_witnesses() {
    let examples = include_str!("../examples/canonical.dotos");
    let values = examples
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with(";;"))
        .collect::<Vec<_>>();

    assert_eq!(values.len(), 12);
    for text in &values[..5] {
        let value = DotosSource::new(text)
            .parse::<z2VVKk>()
            .expect("canonical request decodes");
        assert_eq!(value.to_dotos(), *text);
    }
    for text in &values[5..] {
        let value = DotosSource::new(text)
            .parse::<z2VZMR>()
            .expect("canonical reply decodes");
        assert_eq!(value.to_dotos(), *text);
    }
}
