//
//  RustBridge.swift
//  PassRust
//
//  Created by Ivannikov-EXTERNAL Georgiy on 06.08.2025.
//

import Foundation

final class RustBridge {
    static let shared = RustBridge()

    private init() {
        initStore()
    }

    private func initStore() {
        let dbPath = FileManager.default.temporaryDirectory
            .appendingPathComponent("pass_db")
            .path

        let key = [UInt8](repeating: 42, count: 32)
        key.withUnsafeBufferPointer {
            init_store(dbPath, $0.baseAddress)
        }
    }

    func generatePassword(length: UInt32) -> String {
        guard let raw = generate_password(length) else { return "" }
        defer { free_string(raw) }
        return String(cString: raw)
    }

    func encrypt(_ plaintext: String) -> String {
        guard let raw = encrypt_password(plaintext) else { return "" }
        defer { free_string(raw) }
        return String(cString: raw)
    }

    func decrypt(_ ciphertext: String) -> String {
        guard let raw = decrypt_password(ciphertext) else { return "" }
        defer { free_string(raw) }
        return String(cString: raw)
    }

    func save(tag: String, password: String) -> Bool {
        if tag_exists(tag) {
            return false
        }
        save_password(tag, password)
        return true
    }

    func load(tag: String) -> String? {
        guard tag_exists(tag), let ptr = get_password(tag) else {
            return nil
        }
        defer { free_string(ptr) }
        return String(cString: ptr)
    }

    func tagExists(_ tag: String) -> Bool {
        tag_exists(tag)
    }
}
