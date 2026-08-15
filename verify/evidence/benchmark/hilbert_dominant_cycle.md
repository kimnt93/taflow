# HilbertDominantCycle benchmark (`HilbertDominantCycle` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.062 | 16.24M | 0.058 | 17.24M | 0.177 | 2.88× | 3.06× |
| 10,000 | 0.584 | 17.12M | 0.575 | 17.39M | 1.019 | 1.75× | 1.77× |
| 100,000 | 6.047 | 16.54M | 5.848 | 17.10M | 9.295 | 1.54× | 1.59× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.137 | 0.187 | 1.37× |
| 1 | 5 | 0.239 | 1.101 | 4.61× |
| 1 | 10 | 0.398 | 1.858 | 4.67× |
| 10 | 1 | 0.045 | 0.164 | 3.66× |
| 10 | 5 | 0.187 | 0.778 | 4.16× |
| 10 | 10 | 0.414 | 1.892 | 4.57× |
| 100 | 1 | 0.057 | 0.178 | 3.12× |
| 100 | 5 | 0.190 | 0.849 | 4.48× |
| 100 | 10 | 0.485 | 2.010 | 4.15× |
| 1,000 | 1 | 0.113 | 0.257 | 2.27× |
| 1,000 | 5 | 0.227 | 1.296 | 5.71× |
| 1,000 | 10 | 0.469 | 2.538 | 5.42× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
