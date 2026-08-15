# UltimateOscillator benchmark (`ULTOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 72.87M | 0.012 | 81.59M | 0.054 | 3.96× | 4.44× |
| 10,000 | 0.115 | 87.33M | 0.113 | 88.11M | 0.199 | 1.74× | 1.75× |
| 100,000 | 1.104 | 90.60M | 1.088 | 91.92M | 1.482 | 1.34× | 1.36× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.114 | 0.131 | 1.15× |
| 1 | 5 | 0.294 | 0.482 | 1.64× |
| 1 | 10 | 0.383 | 1.059 | 2.76× |
| 10 | 1 | 0.046 | 0.094 | 2.03× |
| 10 | 5 | 0.187 | 0.459 | 2.46× |
| 10 | 10 | 0.390 | 1.000 | 2.56× |
| 100 | 1 | 0.046 | 0.100 | 2.17× |
| 100 | 5 | 0.227 | 0.492 | 2.17× |
| 100 | 10 | 0.414 | 1.036 | 2.50× |
| 1,000 | 1 | 0.057 | 0.108 | 1.90× |
| 1,000 | 5 | 0.227 | 0.600 | 2.64× |
| 1,000 | 10 | 0.477 | 1.167 | 2.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
