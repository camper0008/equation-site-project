use openssl::rand::rand_bytes;

fn gen_random_valid_chars() -> Result<[char; 64], openssl::error::ErrorStack> {
    let valid_characters = ['a'..='z', 'A'..='Z', '0'..='9']
        .into_iter()
        .flatten()
        .collect::<Vec<char>>();

    let mut token_buffer = [0; 64];
    rand_bytes(&mut token_buffer)?;

    Ok(token_buffer.map(|n| valid_characters[(n as usize) % valid_characters.len()]))
}

pub fn gen_64_char_random_valid_string() -> Result<String, openssl::error::ErrorStack> {
    let random_chars = gen_random_valid_chars()?;

    Ok(random_chars.into_iter().collect::<String>())
}

pub fn gen_8_char_random_valid_string() -> Result<String, openssl::error::ErrorStack> {
    let random_chars = gen_random_valid_chars()?;

    Ok(random_chars.into_iter().take(8).collect::<String>())
}
