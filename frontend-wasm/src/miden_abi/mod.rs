use miden_core::crypto::hash::RpoDigest;

pub fn parse_import_function_digest(import_name: &str) -> Result<(String, RpoDigest), String> {
    // parse the hex encoded digest from the function name in the angle brackets
    // and the function name (before the angle brackets) example:
    // "miden:tx_kernel/note.get_inputs<0x0000000000000000000000000000>"
    let mut parts = import_name.split('<');
    let function_name = parts.next().unwrap();
    let digest = parts
        .next()
        .and_then(|s| s.strip_suffix('>'))
        .ok_or_else(|| "Import name parsing error: missing closing angle bracket in import name")?;
    Ok((
        function_name.to_string(),
        RpoDigest::try_from(digest).map_err(|e| e.to_string())?,
    ))
}
