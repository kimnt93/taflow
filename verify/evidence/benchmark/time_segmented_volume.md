# TimeSegmentedVolume benchmark (`TSV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 99.14M | 0.009 | 111.49M | 0.220 | 21.77× | 24.48× |
| 10,000 | 0.057 | 174.35M | 0.054 | 183.90M | 0.820 | 14.30× | 15.08× |
| 100,000 | 0.534 | 187.26M | 0.505 | 198.18M | 7.071 | 13.24× | 14.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.268 | 3.91× |
| 1 | 5 | 0.326 | 1.123 | 3.44× |
| 1 | 10 | 0.509 | 2.299 | 4.52× |
| 10 | 1 | 0.057 | 0.221 | 3.88× |
| 10 | 5 | 0.264 | 1.296 | 4.90× |
| 10 | 10 | 0.499 | 2.321 | 4.65× |
| 100 | 1 | 0.056 | 0.227 | 4.07× |
| 100 | 5 | 0.228 | 1.356 | 5.94× |
| 100 | 10 | 0.498 | 2.482 | 4.99× |
| 1,000 | 1 | 0.061 | 0.278 | 4.58× |
| 1,000 | 5 | 0.253 | 1.599 | 6.31× |
| 1,000 | 10 | 0.566 | 2.974 | 5.25× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
