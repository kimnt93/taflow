# ParabolicMovingAverageStop benchmark

Correctness: **MISMATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.025 | 40.05M | 0.024 | 42.16M | 3.158 | 126.50× | 133.16× |
| 10,000 | 0.191 | 52.38M | 0.197 | 50.86M | 17.074 | 89.44× | 86.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.346 | 0.410 | 1.19× |
| 1 | 5 | 0.336 | 0.905 | 2.69× |
| 1 | 10 | 0.597 | 1.460 | 2.44× |
| 10 | 1 | 0.056 | 1.785 | 31.71× |
| 10 | 5 | 0.256 | 8.180 | 31.98× |
| 10 | 10 | 0.555 | 16.239 | 29.25× |
| 100 | 1 | 0.069 | 1.760 | 25.69× |
| 100 | 5 | 0.289 | 9.062 | 31.40× |
| 100 | 10 | 0.667 | 19.162 | 28.71× |
| 1,000 | 1 | 0.093 | 3.238 | 34.73× |
| 1,000 | 5 | 0.275 | 16.728 | 60.73× |
| 1,000 | 10 | 0.793 | 34.005 | 42.88× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | Reference full µs | vs full |
|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.477 | 0.310 | 3.23M | 3891.091 | 12550.29× |
| 1,500 | 10 | 2.487 | 1.257 | 7.95M | 3869.817 | 3077.88× |
| 1,500 | 100 | 6.494 | 4.550 | 21.98M | 4064.931 | 893.39× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | Reference vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.77M | 13.10M | 1.00× | 766.95K | 719.41K | 1.00× | 276.78K |
| 5 | 8.12M | 13.81M | 1.05× | 1.15M | 1.32M | 1.83× | 284.48K |
| 10 | 15.49M | 17.83M | 1.36× | 1.16M | 1.26M | 1.75× | 273.45K |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
