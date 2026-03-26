#![no_std]
// Gộp tất cả các thư viện cần thiết vào một dòng cho gọn
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, String, Symbol};

const GREETING_KEY: Symbol = symbol_short!("GREET");

#[contract]
pub struct StellarGreetings;

#[contractimpl]
impl StellarGreetings {
    // --- PHẦN 1: LỜI CHÀO CHUNG (Giống trong ảnh của m) ---
    
    pub fn set_greeting(env: Env, new_greeting: String) {
        env.storage().instance().set(&GREETING_KEY, &new_greeting);
    }

    pub fn get_greeting(env: Env) -> String {
        env.storage()
            .instance()
            .get(&GREETING_KEY)
            .unwrap_or_else(|| String::from_str(&env, "No greeting yet!"))
    }

    // --- PHẦN 2: SỔ LƯU BÚT CÁ NHÂN (Mỗi người một lời nhắn) ---

    pub fn sign_guestbook(env: Env, user: Address, message: String) {
        // Xác thực danh tính người ký
        user.require_auth();
        
        // Lưu tin nhắn riêng biệt theo địa chỉ ví (Address)
        env.storage().persistent().set(&user, &message);
    }

    pub fn get_message(env: Env, user: Address) -> String {
        env.storage()
            .persistent()
            .get(&user)
            .unwrap_or_else(|| String::from_str(&env, "Chưa có lời nhắn nào từ ví này!"))
    }
}