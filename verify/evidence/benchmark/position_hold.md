# PositionHold benchmark (`nonzero position hold` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 43.94M | 0.018 | 55.80M | 0.122 | 5.36× | 6.81× |
| 10,000 | 0.125 | 80.09M | 0.118 | 84.43M | 1.169 | 9.36× | 9.87× |
| 100,000 | 1.118 | 89.43M | 1.062 | 94.17M | 11.553 | 10.33× | 10.88× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.160 | 0.151 | 0.94× |
| 1 | 5 | 0.388 | 0.296 | 0.76× |
| 1 | 10 | 0.529 | 0.600 | 1.13× |
| 10 | 1 | 0.060 | 0.059 | 0.98× |
| 10 | 5 | 0.275 | 0.296 | 1.08× |
| 10 | 10 | 0.576 | 0.610 | 1.06× |
| 100 | 1 | 0.061 | 0.070 | 1.14× |
| 100 | 5 | 0.272 | 0.343 | 1.26× |
| 100 | 10 | 0.574 | 0.729 | 1.27× |
| 1,000 | 1 | 0.076 | 0.179 | 2.36× |
| 1,000 | 5 | 0.295 | 0.881 | 2.98× |
| 1,000 | 10 | 0.564 | 1.778 | 3.15× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
