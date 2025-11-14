# ⚡ Fastest TRON Vanity Generator (Rust)

A blazing-fast **TRX Vanity Address Generator** written in Rust 🚀
Optimized for maximum throughput using hybrid CPU processing.

---

## 🔥 Real Speed Test (Benchmarks)

| 🧠 CPU                  | ⚡ Speed (addr/s)     | 📦 Batch Size |
| ----------------------- | -------------------- | ------------- |
| **AMD EPYC**            | **2,000,000 addr/s** | 1,000,000     |
| **AMD Ryzen 9 7950X3D** | **1,000,000 addr/s** | 200,000       |
| **AMD Ryzen 7 9800X3D** | **490,000 addr/s**   | 100,000       |

---

## ▶️ Run

```bash
cargo run --release
```

---

## 📝 Output Format (Results Saved)

All successful matches are saved into **`r_nice.txt`** 🗂️
Each result is written in the following format:

```
TRX Address | Pattern | Private Key
```

Example:

```
TXY4USDTxxxx | @*USDT | 7f3a1d9b0c...
```

---

## ⚙️ Configuration

All patterns are configured inside **`patterns.json`**.

### 🎯 Pattern Options

#### 🔁 Same-character endings:

```json
"same_four": true,   // 4 identical characters at the end
"same_five": true,   // 5 identical characters at the end
"same_six": true     // 6 identical characters at the end
```

#### 🔤 Word-based matching:

```json
"words": [
    "USDT",     // strict uppercase match
    "*USDT",    // case-insensitive
    "@USDT",    // strict uppercase, must follow digit or vowel
    "@*USDT"    // case-insensitive, must follow digit or vowel
]
```

**Legend:**

* `WORD` → exact-case match
* `*WORD` → any letter-case
* `@WORD` → digit/vowel before the word
* `@*WORD` → both combined

---

## ⚡ Batch Size Optimization

You can tune the generator performance manually.

Edit **`src/hybrid.rs`**:

```rust
const BATCH_SIZE: usize = 100_000;
```

Different hardware benefits from different batch sizes — experiment to find the optimal value.

---

## ⭐ Features

* 🔥 Ultra-fast TRON vanity generation
* 🧵 Multi-threaded hybrid engine
* 🎚️ Pattern-based filtering
* 🔡 Wildcards & semantic prefixes
* 🏎️ Rust-level performance

---

## 🤝 Contributions

Pull requests and improvements are welcome!

---

## 📄 License

MIT License © 2025
