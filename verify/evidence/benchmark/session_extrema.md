# SessionExtrema benchmark (`explicit-session extrema` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 113.98M | 0.007 | 152.99M | 0.515 | 58.68× | 78.77× |
| 10,000 | 0.056 | 179.82M | 0.049 | 204.95M | 5.041 | 90.65× | 103.32× |
| 100,000 | 0.542 | 184.43M | 0.476 | 209.90M | 50.709 | 93.52× | 106.44× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.109 | 0.092 | 0.85× |
| 1 | 5 | 0.218 | 0.336 | 1.54× |
| 1 | 10 | 0.385 | 0.649 | 1.69× |
| 10 | 1 | 0.049 | 0.072 | 1.48× |
| 10 | 5 | 0.179 | 0.340 | 1.90× |
| 10 | 10 | 0.432 | 0.679 | 1.57× |
| 100 | 1 | 0.047 | 0.128 | 2.72× |
| 100 | 5 | 0.212 | 0.609 | 2.88× |
| 100 | 10 | 0.417 | 1.151 | 2.76× |
| 1,000 | 1 | 0.050 | 0.570 | 11.32× |
| 1,000 | 5 | 0.204 | 2.961 | 14.54× |
| 1,000 | 10 | 0.430 | 5.810 | 13.52× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
