# CandleClosingMarubozu benchmark (`CDLCLOSINGMARUBOZU` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 140.56M | 0.004 | 254.85M | 0.036 | 5.10× | 9.25× |
| 10,000 | 0.103 | 97.47M | 0.098 | 102.18M | 0.129 | 1.25× | 1.31× |
| 100,000 | 0.997 | 100.30M | 1.080 | 92.58M | 1.107 | 1.11× | 1.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.057 | 0.130 | 2.27× |
| 1 | 5 | 0.286 | 0.631 | 2.20× |
| 1 | 10 | 0.525 | 1.112 | 2.12× |
| 10 | 1 | 0.069 | 0.113 | 1.64× |
| 10 | 5 | 0.366 | 0.602 | 1.65× |
| 10 | 10 | 0.474 | 1.108 | 2.34× |
| 100 | 1 | 0.059 | 0.104 | 1.76× |
| 100 | 5 | 0.324 | 0.688 | 2.12× |
| 100 | 10 | 0.529 | 1.092 | 2.06× |
| 1,000 | 1 | 0.094 | 0.125 | 1.34× |
| 1,000 | 5 | 0.344 | 0.750 | 2.18× |
| 1,000 | 10 | 0.451 | 1.094 | 2.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
