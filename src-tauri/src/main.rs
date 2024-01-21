use bitcoin::PrivateKey;
use bitcoin::network::Network;
use secp256k1::{Secp256k1, SecretKey};
use argon2::{Argon2, password_hash::{PasswordHasher, SaltString}};
use sha3::{Sha3_256, Digest};
use tauri::Builder;
use bs58;


#[tauri::command]
fn generate_bitcoin_private_key(provided_code: String, password_string: String) -> Result<String, String> {
    // 既存の入力検証処理
    if provided_code.len() < 16 {
        return Err("Provided code must be at least 16 characters long".to_string());
    }
    if password_string.len() < 8 {
        return Err("Password must be at least 8 characters long".to_string());
    }
    if bs58::decode(&provided_code).into_vec().is_err() || bs58::decode(&password_string).into_vec().is_err() {
        return Err("Provided code or password is not in Base58 format".to_string());
    }

    let password_bytes = password_string.as_bytes();
    let salt = SaltString::encode_b64(password_bytes).expect("Failed to create salt");
    let concatenated = format!("{}{}", provided_code, password_string);
    let argon2 = Argon2::default();
    let argon2_hash = argon2.hash_password(concatenated.as_bytes(), &salt)
        .expect("Failed to hash with Argon2")
        .to_string();

    // Argon2ハッシュの最初の32バイトを使用して秘密鍵を生成
    let _secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(&argon2_hash.as_bytes()[..32]) // 最初の32バイトを使用
        .map_err(|e| e.to_string())?;

    // 秘密鍵をWIF形式に変換
    let private_key = PrivateKey {
        compressed: true,
        network: Network::Bitcoin,
        inner: secret_key,
    };

    Ok(private_key.to_wif())
}




#[tauri::command]
fn generate_private_key(provided_code: String, password_string: String) -> Result<String, String> {
    if provided_code.len() < 16 {
        return Err("Provided code must be at least 16 characters long".to_string());
    }
    if password_string.len() < 8 {
        return Err("Password must be at least 8 characters long".to_string());
    }

    // Base58形式であるかどうかをチェック
    if bs58::decode(&provided_code).into_vec().is_err() || bs58::decode(&password_string).into_vec().is_err() {
        return Err("Provided code or password is not in Base58 format".to_string());
    }
    // password_stringをバイト列にエンコードし、最初の16バイトをソルトとして使用
    let password_bytes = password_string.as_bytes();
    let salt = SaltString::encode_b64(password_bytes).expect("Failed to create salt");

    // provided_codeとpassword_stringを連結
    let concatenated = format!("{}{}", provided_code, password_string);

    // Argon2idでキーストレッチング
    let argon2 = Argon2::default();
    let argon2_hash = argon2.hash_password(concatenated.as_bytes(), &salt)
        .expect("Failed to hash with Argon2")
        .to_string();

    // SHA-3で追加ハッシュ化
    let mut sha3_hasher = Sha3_256::new();
    sha3_hasher.update(argon2_hash.as_bytes());
    let sha3_hash = sha3_hasher.finalize();

    Ok(format!("{:x}", sha3_hash))
}

fn main() {
    Builder::default()
        .invoke_handler(tauri::generate_handler![generate_private_key,generate_bitcoin_private_key])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}