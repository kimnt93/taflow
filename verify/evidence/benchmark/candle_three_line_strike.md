# CandleThreeLineStrike benchmark (`CDL3LINESTRIKE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 127.80M | 0.005 | 219.54M | 0.034 | 4.34× | 7.45× |
| 10,000 | 0.084 | 118.56M | 0.073 | 136.94M | 0.113 | 1.34× | 1.55× |
| 100,000 | 0.857 | 116.70M | 0.820 | 121.98M | 0.782 | 0.91× | 0.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.108 | 1.23× |
| 1 | 5 | 0.359 | 0.507 | 1.41× |
| 1 | 10 | 0.405 | 0.965 | 2.38× |
| 10 | 1 | 0.046 | 0.085 | 1.84× |
| 10 | 5 | 0.180 | 0.426 | 2.36× |
| 10 | 10 | 0.409 | 0.954 | 2.33× |
| 100 | 1 | 0.048 | 0.110 | 2.28× |
| 100 | 5 | 0.217 | 0.453 | 2.08× |
| 100 | 10 | 0.433 | 0.919 | 2.12× |
| 1,000 | 1 | 0.052 | 0.095 | 1.85× |
| 1,000 | 5 | 0.194 | 0.477 | 2.46× |
| 1,000 | 10 | 0.464 | 1.028 | 2.22× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
