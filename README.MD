![alt text](ec86996c-71a6-4d9f-a66f-921ce43f4af9.jpeg)
---

# 🧠 NeoDex — Your Offline AI File Assistant

NeoDex is a local-first AI assistant that **understands your files** via natural language. Fully offline. No spyware. No BS.

---

## ⚙️ Prerequisites

Make sure you've got these installed:

1. 🐍 **Python** (3.10+ recommended) with `pip`
2. 🦀 **Rust** (installed properly via [rust-lang.org](https://www.rust-lang.org/))
3. 🧠 Any code editor (VSCode? Cool — install official Python + Rust extensions)

---

## 📦 Setup Steps

> Run all commands **from the root of the NeoDex project folder.**

---

### 1️⃣ Python setup

```bash
# Create and activate a virtual environment
python3 -m venv venv
source venv/bin/activate  # Windows? Use venv\Scripts\activate

# Install Python deps
pip install -r requirements.txt
```

---

### 2️⃣ Rust setup

```bash
cd rust-core
cargo build  # installs dependencies
cargo run    # starts the file indexer
```

---

### 3️⃣ Embeddings generation

```bash
cd python-core/embeddings
python3 main.py  # Windows users: just 'python main.py' (also... switch to Linux, please.)
```

---

### 4️⃣ Query your files

```bash
cd ../query
python3 query_embeddings.py "your search query in quotes"

# Example:
python3 query_embeddings.py "All text files that contain the word stupid"
```

---

## 📝 Notes

* First Rust scan stores all your files' metadata in SQLite and skips folders listed in `blacklisted_folders.txt`.
* Embedding generation takes time — don’t kill it mid-process unless you want broken vectors.
* NeoDex is **modular** — Rust handles indexing, Python handles embeddings & querying.

---

Want this version saved as your new README? I can dump it into a file.
