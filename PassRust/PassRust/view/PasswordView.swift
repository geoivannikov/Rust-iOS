//
//  PasswordView.swift
//  PassRust
//
//  Created by Ivannikov-EXTERNAL Georgiy on 03.08.2025.
//

import SwiftUI

struct PasswordView: View {
    @StateObject private var viewModel = PasswordViewModel()

    @State private var showSaveAlert = false
    @State private var showLoadAlert = false

    var body: some View {
        NavigationView {
            ScrollView {
                VStack(spacing: 24) {
                    Section {
                        if viewModel.password.isEmpty {
                            Button(action: viewModel.generate) {
                                Label("Generate Password", systemImage: "key.fill")
                                    .frame(maxWidth: .infinity)
                            }
                        } else {
                            VStack(spacing: 8) {
                                labeledValue("Password", viewModel.password)

                                if viewModel.encrypted.isEmpty {
                                    Button("Encrypt", action: viewModel.encrypt)
                                } else {
                                    labeledValue("Encrypted", viewModel.encrypted)

                                    if viewModel.decrypted.isEmpty {
                                        Button("Decrypt", action: viewModel.decrypt)
                                    } else {
                                        labeledValue("Decrypted", viewModel.decrypted)
                                    }
                                }

                                HStack {
                                    Button("💾 Save") {
                                        viewModel.tagToSave = ""
                                        showSaveAlert = true
                                    }

                                    Button("🗑️ Reset", role: .destructive, action: viewModel.reset)
                                }
                            }
                        }
                    } header: {
                        Text("Password Flow")
                            .font(.headline)
                    }

                    Divider()
                    Section {
                        Button("📥 Load Password") {
                            viewModel.tagToLoad = ""
                            showLoadAlert = true
                        }

                        if !viewModel.loadResult.isEmpty {
                            labeledValue("Loaded password", viewModel.loadResult)
                        }
                    } header: {
                        Text("Storage")
                            .font(.headline)
                    }
                    if !viewModel.errorMessage.isEmpty {
                        Text(viewModel.errorMessage)
                            .foregroundColor(.red)
                            .padding(.top)
                    }
                }
                .padding()
            }
            .navigationTitle("PassRust")
        }
        .alert("Enter tag to save password", isPresented: $showSaveAlert) {
            TextField("Tag", text: $viewModel.tagToSave)
            Button("Save", action: viewModel.save)
            Button("Cancel", role: .cancel) {}
        }
        .alert("Enter tag to load password", isPresented: $showLoadAlert) {
            TextField("Tag", text: $viewModel.tagToLoad)
            Button("Load", action: viewModel.load)
            Button("Cancel", role: .cancel) {}
        }
    }

    private func labeledValue(_ label: String, _ value: String) -> some View {
        VStack(alignment: .leading, spacing: 4) {
            Text(label)
                .font(.subheadline)
                .foregroundColor(.secondary)
            Text(value)
                .font(.body)
                .padding(6)
                .frame(maxWidth: .infinity, alignment: .leading)
                .cornerRadius(6)
        }
    }
}
