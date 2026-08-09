# Crossunder benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 197.62M | 0.004 | 261.71M | nan | — | — |
| 10,000 | 0.031 | 322.16M | 0.028 | 359.76M | nan | — | — |
| 100,000 | 0.300 | 333.30M | 0.276 | 361.88M | nan | — | — |
| 1,000,000 | 3.444 | 290.39M | 3.107 | 321.81M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.303 ms**; native kernel **0.281 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.235 | 0.183 | 5.47M | nan | — | — |
| 100,000 | 10 | 1.446 | 0.718 | 13.92M | nan | — | — |
| 100,000 | 1,000 | 5.302 | 4.213 | 237.38M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 213.46M | 204.80M | 1.00× | 2.49M | 2.94M | 1.00× | — |
| 2 | 395.56M | 483.56M | 2.36× | 3.63M | 3.27M | 1.11× | — |
| 4 | 553.37M | 672.52M | 3.28× | 3.20M | 3.33M | 1.13× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
