# CandleUpDownSideGapThreeMethods benchmark (`CDLXSIDEGAP3METHODS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.069 | 14.39M | 0.062 | 16.14M | 0.031 | 0.44× | 0.49× |
| 10,000 | 0.606 | 16.50M | 0.497 | 20.10M | 0.080 | 0.13× | 0.16× |
| 100,000 | 4.836 | 20.68M | 4.860 | 20.58M | 0.596 | 0.12× | 0.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.113 | 0.141 | 1.25× |
| 1 | 5 | 0.428 | 0.444 | 1.04× |
| 1 | 10 | 0.625 | 0.922 | 1.48× |
| 10 | 1 | 0.067 | 0.086 | 1.28× |
| 10 | 5 | 0.318 | 0.417 | 1.31× |
| 10 | 10 | 0.666 | 0.961 | 1.44× |
| 100 | 1 | 0.082 | 0.094 | 1.15× |
| 100 | 5 | 0.327 | 0.434 | 1.33× |
| 100 | 10 | 0.654 | 0.908 | 1.39× |
| 1,000 | 1 | 0.126 | 0.091 | 0.72× |
| 1,000 | 5 | 0.330 | 0.472 | 1.43× |
| 1,000 | 10 | 0.667 | 0.965 | 1.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
