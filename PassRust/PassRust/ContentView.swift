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

    var body: some View {
        VStack(spacing: 20) {
            Text("Generated password:")
            Text(password)
                .padding()

            Text("Encrypted:")
            Text(encrypted)
                .padding()

            Button("Generate & Encrypt") {
                if let raw = generate_password(16) {
                    let pwd = String(cString: raw)
                    password = pwd
                    free_string(raw)

                    if let encRaw = encrypt_password(pwd) {
                        encrypted = String(cString: encRaw)
                        free_string(encRaw)
                    } else {
                        encrypted = "Encryption failed"
                    }
                } else {
                    password = "Generation failed"
                    encrypted = ""
                }
            }
        }
        .padding()
    }
}

#Preview {
    ContentView()
}
