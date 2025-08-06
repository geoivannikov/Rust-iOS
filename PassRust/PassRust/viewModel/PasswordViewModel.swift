//
//  PasswordViewModel.swift
//  PassRust
//
//  Created by Ivannikov-EXTERNAL Georgiy on 06.08.2025.
//

import Foundation
import SwiftUI

final class PasswordViewModel: ObservableObject {
    @Published var password = ""
    @Published var encrypted = ""
    @Published var decrypted = ""
    @Published var loadResult = ""
    @Published var errorMessage = ""

    @Published var tagToSave = ""
    @Published var tagToLoad = ""
    
    private let rustBridge: RustBridge

    init() {
        rustBridge = RustBridge.shared
    }

    func generate() {
        password = rustBridge.generatePassword(length: 16)
    }

    func encrypt() {
        encrypted = rustBridge.encrypt(password)
    }

    func decrypt() {
        decrypted = rustBridge.decrypt(encrypted)
    }

    func save() {
        errorMessage = ""
        guard !tagToSave.isEmpty else { return }

        let success = rustBridge.save(tag: tagToSave, password: password)
        if !success {
            errorMessage = "❗️Tag already exists"
        }
    }

    func load() {
        errorMessage = ""
        guard !tagToLoad.isEmpty else { return }

        if let result = rustBridge.load(tag: tagToLoad) {
            loadResult = result
        } else {
            errorMessage = "❗️Tag not found or load failed"
        }
    }

    func reset() {
        password = ""
        encrypted = ""
        decrypted = ""
        loadResult = ""
        errorMessage = ""
    }
}
