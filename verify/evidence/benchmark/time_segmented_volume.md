# TimeSegmentedVolume benchmark (`TSV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 105.56M | 0.008 | 122.33M | 0.197 | 20.84× | 24.16× |
| 10,000 | 0.051 | 194.47M | 0.050 | 201.48M | 0.788 | 15.33× | 15.88× |
| 100,000 | 0.483 | 206.83M | 0.465 | 215.18M | 6.734 | 13.93× | 14.49× |
| 1,000,000 | 5.405 | 185.01M | 4.699 | 212.81M | 65.330 | 12.09× | 13.90× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.127 | 0.270 | 2.13× |
| 1 | 5 | 0.290 | 1.244 | 4.29× |
| 1 | 10 | 0.503 | 2.376 | 4.73× |
| 10 | 1 | 0.051 | 0.216 | 4.25× |
| 10 | 5 | 0.242 | 1.218 | 5.03× |
| 10 | 10 | 0.462 | 2.413 | 5.23× |
| 100 | 1 | 0.051 | 0.223 | 4.34× |
| 100 | 5 | 0.242 | 1.235 | 5.09× |
| 100 | 10 | 0.530 | 2.395 | 4.52× |
| 1,000 | 1 | 0.056 | 0.280 | 5.01× |
| 1,000 | 5 | 0.235 | 1.683 | 7.16× |
| 1,000 | 10 | 0.528 | 3.027 | 5.73× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
