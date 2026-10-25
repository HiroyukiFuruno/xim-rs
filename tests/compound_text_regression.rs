use xim_ctext::compound_text_to_utf8;

#[test]
fn japanese_preedit_does_not_repeat_consumed_input() {
    let compound_text = [
        27, 36, 40, 66, 0x24, 0x4b, 0x24, 0x5b, 0x24, 0x73, 0x24, 0x34,
    ];

    assert_eq!(compound_text_to_utf8(&compound_text).unwrap(), "にほんご");
}
