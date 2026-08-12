# CandleShortLine benchmark (`CDLSHORTLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.97M | 0.015 | 66.60M | 0.037 | 1.91× | 2.49× |
| 10,000 | 0.153 | 65.43M | 0.153 | 65.15M | 0.208 | 1.36× | 1.36× |
| 100,000 | 1.611 | 62.06M | 1.628 | 61.44M | 1.811 | 1.12× | 1.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.134 | 1.41× |
| 1 | 5 | 0.344 | 0.463 | 1.35× |
| 1 | 10 | 0.532 | 0.903 | 1.70× |
| 10 | 1 | 0.054 | 0.087 | 1.60× |
| 10 | 5 | 0.272 | 0.499 | 1.84× |
| 10 | 10 | 0.579 | 0.937 | 1.62× |
| 100 | 1 | 0.057 | 0.099 | 1.73× |
| 100 | 5 | 0.302 | 0.496 | 1.64× |
| 100 | 10 | 0.624 | 0.959 | 1.54× |
| 1,000 | 1 | 0.074 | 0.123 | 1.67× |
| 1,000 | 5 | 0.283 | 0.557 | 1.97× |
| 1,000 | 10 | 0.599 | 1.167 | 1.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
