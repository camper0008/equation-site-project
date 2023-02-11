use openssl::rand::rand_bytes;

fn gen_random_valid_chars() -> Result<[char; 64], openssl::error::ErrorStack> {
    const VALID_CHARACTERS: [char; 62] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1',
        '2', '3', '4', '5', '6', '7', '8', '9',
    ];

    let mut token_buffer = [0; 64];
    rand_bytes(&mut token_buffer)?;

    Ok(token_buffer.map(|n| VALID_CHARACTERS[(n as usize) % VALID_CHARACTERS.len()]))
}

pub fn gen_64_char_random_valid_string() -> Result<String, openssl::error::ErrorStack> {
    let random_chars = gen_random_valid_chars()?;

    Ok(random_chars.into_iter().collect::<String>())
}

pub fn gen_8_char_random_valid_string() -> Result<String, openssl::error::ErrorStack> {
    let random_chars = gen_random_valid_chars()?;

    Ok(random_chars.into_iter().take(8).collect::<String>())
}
