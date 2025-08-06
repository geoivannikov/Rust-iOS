//
//  ContentView.swift
//  PassRust
//
//  Created by Ivannikov-EXTERNAL Georgiy on 03.08.2025.
//

import SwiftUI

struct ContentView: View {
    @State private var password = ""
    @State private var encrypted = ""
    @State private var decrypted = ""

    var body: some View {
        VStack(spacing: 20) {
            if password.isEmpty {
                Button("Generate Password") {
                    if let raw = generate_password(16) {
                        let pwd = String(cString: raw)
                        password = pwd
                        free_string(raw)
                    }
                }
            } else {
                Text("Password:")
                    .bold()
                Text(password)
                    .padding()

                if encrypted.isEmpty {
                    Button("Encrypt Password") {
                        if let encRaw = encrypt_password(password) {
                            encrypted = String(cString: encRaw)
                            free_string(encRaw)
                        }
                    }
                } else {
                    Text("Encrypted:")
                        .bold()
                    Text(encrypted)
                        .padding()

                    if decrypted.isEmpty {
                        Button("Decrypt Password") {
                            if let decRaw = decrypt_password(encrypted) {
                                decrypted = String(cString: decRaw)
                                free_string(decRaw)
                            }
                        }
                    } else {
                        Text("Decrypted:")
                            .bold()
                        Text(decrypted)
                            .padding()
                    }
                }

                Button("Reset") {
                    password = ""
                    encrypted = ""
                    decrypted = ""
                }
                .foregroundColor(.red)
            }
        }
        .padding()
    }
}

#Preview {
    ContentView()
}
