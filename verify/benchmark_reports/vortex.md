# Vortex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 52.20M | 0.017 | 57.97M | nan | — | — |
| 10,000 | 0.160 | 62.38M | 0.155 | 64.43M | nan | — | — |
| 100,000 | 1.545 | 64.74M | 1.489 | 67.16M | nan | — | — |
| 1,000,000 | 25.811 | 38.74M | 16.007 | 62.47M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.532 ms**; native kernel **1.529 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.317 | 0.255 | 3.92M | nan | — | — |
| 100,000 | 10 | 2.025 | 1.208 | 8.28M | nan | — | — |
| 100,000 | 1,000 | 24.204 | 18.605 | 53.75M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 56.34M | 55.96M | 1.00× | 2.35M | 2.39M | 1.00× | — |
| 2 | 93.10M | 110.04M | 1.97× | 2.25M | 2.43M | 1.02× | — |
| 4 | 115.87M | 123.97M | 2.22× | 2.29M | 2.38M | 1.00× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
