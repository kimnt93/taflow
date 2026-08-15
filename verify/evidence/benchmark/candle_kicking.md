# CandleKicking benchmark (`CDLKICKING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 100.96M | 0.006 | 154.32M | 0.046 | 4.63× | 7.07× |
| 10,000 | 0.123 | 81.26M | 0.121 | 82.95M | 0.204 | 1.65× | 1.69× |
| 100,000 | 1.171 | 85.39M | 1.175 | 85.11M | 1.519 | 1.30× | 1.29× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.107 | 1.57× |
| 1 | 5 | 0.258 | 0.466 | 1.81× |
| 1 | 10 | 0.421 | 0.889 | 2.11× |
| 10 | 1 | 0.044 | 0.085 | 1.93× |
| 10 | 5 | 0.177 | 0.456 | 2.57× |
| 10 | 10 | 0.430 | 0.940 | 2.18× |
| 100 | 1 | 0.040 | 0.087 | 2.16× |
| 100 | 5 | 0.191 | 0.448 | 2.35× |
| 100 | 10 | 0.428 | 0.968 | 2.26× |
| 1,000 | 1 | 0.061 | 0.109 | 1.78× |
| 1,000 | 5 | 0.190 | 0.505 | 2.65× |
| 1,000 | 10 | 0.453 | 1.086 | 2.40× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
