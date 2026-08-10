# DecayLinear benchmark (`linear decay weighted mean` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 155.52M | 0.006 | 173.07M | 0.083 | 12.84× | 14.29× |
| 10,000 | 0.039 | 259.30M | 0.036 | 281.14M | 0.278 | 7.21× | 7.82× |
| 100,000 | 0.353 | 283.63M | 0.328 | 305.27M | 2.186 | 6.20× | 6.67× |
| 1,000,000 | 3.698 | 270.42M | 3.265 | 306.29M | 21.613 | 5.84× | 6.62× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.139 | 1.28× |
| 1 | 5 | 0.242 | 0.554 | 2.29× |
| 1 | 10 | 0.462 | 1.133 | 2.45× |
| 10 | 1 | 0.051 | 0.107 | 2.11× |
| 10 | 5 | 0.213 | 0.528 | 2.48× |
| 10 | 10 | 0.474 | 1.109 | 2.34× |
| 100 | 1 | 0.051 | 0.151 | 2.95× |
| 100 | 5 | 0.226 | 0.710 | 3.15× |
| 100 | 10 | 0.482 | 1.474 | 3.06× |
| 1,000 | 1 | 0.056 | 0.162 | 2.88× |
| 1,000 | 5 | 0.231 | 0.778 | 3.37× |
| 1,000 | 10 | 0.503 | 1.671 | 3.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
