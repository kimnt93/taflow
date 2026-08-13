# TripleTopBottom benchmark (`TripleTopBottom` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.056 | 17.91M | 0.048 | 20.72M | 0.218 | 3.90× | 4.52× |
| 10,000 | 0.386 | 25.91M | 0.376 | 26.58M | 1.320 | 3.42× | 3.51× |
| 100,000 | 3.656 | 27.35M | 3.723 | 26.86M | 12.444 | 3.40× | 3.34× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.128 | 0.223 | 1.75× |
| 1 | 5 | 0.391 | 0.826 | 2.11× |
| 1 | 10 | 0.671 | 1.651 | 2.46× |
| 10 | 1 | 0.077 | 0.166 | 2.14× |
| 10 | 5 | 0.330 | 1.163 | 3.52× |
| 10 | 10 | 0.665 | 1.703 | 2.56× |
| 100 | 1 | 0.086 | 0.174 | 2.02× |
| 100 | 5 | 0.313 | 1.146 | 3.66× |
| 100 | 10 | 0.680 | 1.822 | 2.68× |
| 1,000 | 1 | 0.112 | 0.296 | 2.63× |
| 1,000 | 5 | 0.313 | 1.751 | 5.59× |
| 1,000 | 10 | 0.700 | 2.968 | 4.24× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
