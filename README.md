Here is a clean, polished, GitHub-ready **README.md** with emojis, formatted beautifully and professionally:

---

# ⚡ Fastest TRON Vanity Generator (Rust)

A blazing-fast **TRX Vanity Address Generator** written in Rust 🚀
Optimized for maximum throughput using hybrid CPU processing.

---

## 🔥 Real Speed Test (Benchmarks)

| 🧠 CPU                  | ⚡ Speed (addr/s)     | 📦 Batch Size |
| ----------------------- | -------------------- | ------------- |
| **AMD EPYC**            | **2,000,000 addr/s** | 1,000,000     |
| **AMD Ryzen 9 7950X3D** | **1,000,000 addr/s** | 200,000       |
| **Ryzen 7 9800X3D**     | **490,000 addr/s**   | 100,000       |

---

## ▶️ Run

```bash
cargo run --release
```

---

## ⚙️ Configuration

All patterns are configured inside **`patterns.json`**.

### 🎯 Pattern Options

#### 🔁 Same-character endings:

```json
"same_four": true    // 4 identical characters at the end
"same_five": true    // 5 identical characters at the end
"same_six": true     // 6 identical characters at the end
```

#### 🔤 Word-based matching:

```json
"words": [
    "USDT",     // strict uppercase match
    "*USDT",    // case-insensitive (Usdt, uSdT, etc.)
    "@USDT",    // strict uppercase, must follow digit or vowel (beautiful readable layout)
    "@*USDT"    // case-insensitive, must follow digit or vowel
]
```

**Legend:**

* `WORD` → match in exact case
* `*WORD` → match in any letter-case
* `@WORD` → match only if before the word is a **digit or vowel**
* `@*WORD` → both of the above combined

Perfect for finding beautifully readable TRX vanity addresses 😎

---

## ⚡ Batch Size Optimization

You can tune the generator performance manually.

Edit **`src/hybrid.rs`**:

```rust
const BATCH_SIZE: usize = 100_000;
```

### 🧪 Tip

Different CPUs have different optimal batch sizes.
Run benchmarks to find the sweet spot for your hardware.

---

## ⭐ Features

* 🔥 Extremely fast TRON vanity generation
* 🧵 Multi-threaded hybrid search
* 📝 Flexible pattern system
* 🧩 Supports wildcards and semantic prefixes
* 🏎️ Fully optimized Rust performance

---

## 🤝 Contributions

Pull requests and improvements are welcome!

---

## 📄 License

MIT License © 2025

---

If you want, I can also:
✅ generate a logo/banner for the repo
✅ build a full GitHub project structure
✅ generate patterns.json example
Just say the word!
