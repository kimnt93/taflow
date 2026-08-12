# Supertrend benchmark (`supertrend` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 44.93M | 0.020 | 50.74M | 1.538 | 69.10× | 78.03× |
| 10,000 | 0.192 | 52.12M | 0.172 | 58.11M | 2.481 | 12.93× | 14.41× |
| 100,000 | 2.016 | 49.60M | 1.776 | 56.31M | 12.351 | 6.13× | 6.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.296 | 3.18× |
| 1 | 5 | 0.299 | 1.164 | 3.89× |
| 1 | 10 | 0.486 | 2.467 | 5.07× |
| 10 | 1 | 0.057 | 1.790 | 31.66× |
| 10 | 5 | 0.269 | 9.416 | 35.05× |
| 10 | 10 | 0.555 | 17.882 | 32.20× |
| 100 | 1 | 0.067 | 1.647 | 24.55× |
| 100 | 5 | 0.283 | 8.404 | 29.66× |
| 100 | 10 | 0.552 | 16.924 | 30.66× |
| 1,000 | 1 | 0.084 | 1.753 | 20.91× |
| 1,000 | 5 | 0.288 | 9.581 | 33.22× |
| 1,000 | 10 | 0.584 | 19.557 | 33.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
