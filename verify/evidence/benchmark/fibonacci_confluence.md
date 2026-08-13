# FibonacciConfluence benchmark (`FibConfluence` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 4.842 | 206.52K | 4.606 | 217.11K | 1.766 | 0.36× | 0.38× |
| 10,000 | 47.291 | 211.46K | 46.275 | 216.10K | 19.220 | 0.41× | 0.42× |
| 100,000 | 479.364 | 208.61K | 468.925 | 213.25K | 176.495 | 0.37× | 0.38× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.131 | 0.207 | 1.58× |
| 1 | 5 | 0.417 | 0.916 | 2.19× |
| 1 | 10 | 0.617 | 1.854 | 3.01× |
| 10 | 1 | 0.069 | 0.172 | 2.50× |
| 10 | 5 | 0.307 | 0.827 | 2.70× |
| 10 | 10 | 0.638 | 1.860 | 2.92× |
| 100 | 1 | 0.317 | 0.288 | 0.91× |
| 100 | 5 | 0.524 | 1.441 | 2.75× |
| 100 | 10 | 0.858 | 3.093 | 3.60× |
| 1,000 | 1 | 4.839 | 2.250 | 0.46× |
| 1,000 | 5 | 6.224 | 10.643 | 1.71× |
| 1,000 | 10 | 8.735 | 21.270 | 2.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
