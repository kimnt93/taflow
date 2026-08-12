# AbsolutePriceOscillator benchmark (`APO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 116.89M | 0.007 | 134.87M | 0.044 | 5.10× | 5.89× |
| 10,000 | 0.052 | 191.65M | 0.048 | 207.18M | 0.077 | 1.48× | 1.60× |
| 100,000 | 0.474 | 211.13M | 0.437 | 228.90M | 0.473 | 1.00× | 1.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.131 | 0.105 | 0.80× |
| 1 | 5 | 0.325 | 0.501 | 1.54× |
| 1 | 10 | 0.547 | 1.050 | 1.92× |
| 10 | 1 | 0.052 | 0.093 | 1.80× |
| 10 | 5 | 0.248 | 0.466 | 1.88× |
| 10 | 10 | 0.506 | 1.087 | 2.15× |
| 100 | 1 | 0.058 | 0.094 | 1.62× |
| 100 | 5 | 0.248 | 0.528 | 2.13× |
| 100 | 10 | 0.520 | 1.107 | 2.13× |
| 1,000 | 1 | 0.066 | 0.111 | 1.68× |
| 1,000 | 5 | 0.250 | 0.512 | 2.05× |
| 1,000 | 10 | 0.543 | 1.064 | 1.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
