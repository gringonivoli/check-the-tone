# CTT - Check The Tone

[![Crates.io](https://img.shields.io/crates/v/check-the-tone.svg)](https://crates.io/crates/check-the-tone)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Dependency Status](https://deps.rs/repo/github/gringonivoli/check-the-tone/status.svg)](https://deps.rs/repo/github/gringonivoli/check-the-tone)

A command-line tool that analyzes the tone of your messages and provides improved versions using local Large Language Models via Ollama.

## About

CTT (Check The Tone) helps you communicate more effectively by analyzing whether your messages are polite, clear, friendly, and collaborative. It then provides a revised version that maintains your intent while improving the tone.

Perfect for:
- Reviewing emails before sending
- Improving chat messages
- Ensuring professional communication
- Learning better communication patterns

## Features

- 🤖 Uses local LLMs via Ollama (privacy-first, no data sent to external servers)
- ⚡ Fast and simple CLI interface
- 🎯 Provides both tone analysis and revised messages
- 🔧 Configurable model and Ollama server settings

## Prerequisites

- [Ollama](https://ollama.ai/) installed and running
- A compatible model pulled (default: `llama3.1:latest`)

## Installation

```bash
cargo install check-the-tone
```

Make sure you have [Ollama](https://ollama.ai/) installed and running with a model pulled.

## Usage

Basic usage:

```bash
ctt -m "your message here"
```

### Options

- `-m, --msg <MESSAGE>` - The message to analyze (required)
- `-M, --model <MODEL>` - LLM model to use (default: `llama3.1:latest`)
- `-u, --url <URL>` - Ollama server URL (default: `http://localhost`)
- `-p, --port <PORT>` - Ollama server port (default: `11434`)

### Examples

```bash
# Check a simple message
ctt -m "fix this bug now"

# Use a different model
ctt -m "Can you review my PR?" -M "mistral:latest"

# Connect to a remote Ollama instance
ctt -m "Hello team" -u "http://192.168.1.100" -p 11434
```
