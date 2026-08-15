# PremiumDiscount benchmark (`rolling premium-discount zone` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.95M | 0.019 | 53.20M | 3.290 | 157.76× | 175.03× |
| 10,000 | 0.271 | 36.85M | 0.262 | 38.13M | 34.936 | 128.73× | 133.22× |
| 100,000 | 2.624 | 38.11M | 2.652 | 37.71M | 334.450 | 127.47× | 126.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.175 | 0.114 | 0.65× |
| 1 | 5 | 0.216 | 0.501 | 2.32× |
| 1 | 10 | 0.434 | 0.982 | 2.26× |
| 10 | 1 | 0.042 | 0.127 | 3.03× |
| 10 | 5 | 0.192 | 0.615 | 3.21× |
| 10 | 10 | 0.424 | 1.332 | 3.14× |
| 100 | 1 | 0.047 | 0.425 | 9.13× |
| 100 | 5 | 0.192 | 2.151 | 11.18× |
| 100 | 10 | 0.481 | 4.484 | 9.32× |
| 1,000 | 1 | 0.086 | 3.510 | 40.75× |
| 1,000 | 5 | 0.218 | 17.908 | 82.06× |
| 1,000 | 10 | 0.569 | 35.998 | 63.23× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
