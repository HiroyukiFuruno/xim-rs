const HIRAGANA_WIRE: &[u8] = &[
    0x1b, 0x24, 0x28, 0x42, 0x24, 0x4b, 0x24, 0x5b, 0x24, 0x73, 0x24, 0x34,
];
const MIXED_WIRE: &[u8] = &[
    0x6e, 0x69, 0x68, 0x6f, 0x6e, 0x67, 0x6f, 0x1b, 0x24, 0x28, 0x42, 0x46, 0x7c, 0x4b, 0x5c, 0x38,
    0x6c,
];

// Xlib実wireを使い、同じ実装のencode/decodeで誤りが相殺されるのを防ぐ。
#[test]
fn decodes_xlib_compound_text_hiragana() -> Result<(), xim_ctext::DecodeError> {
    assert_eq!(xim_ctext::compound_text_to_utf8(HIRAGANA_WIRE)?, "にほんご");
    Ok(())
}

#[test]
fn decodes_xlib_compound_text_mixed_ascii_and_japanese() -> Result<(), xim_ctext::DecodeError> {
    assert_eq!(
        xim_ctext::compound_text_to_utf8(MIXED_WIRE)?,
        "nihongo日本語"
    );
    Ok(())
}
