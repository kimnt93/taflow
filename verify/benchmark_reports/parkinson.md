# Parkinson benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 61.85M | 0.015 | 66.74M | nan | — | — |
| 10,000 | 0.135 | 74.04M | 0.135 | 73.94M | nan | — | — |
| 100,000 | 1.306 | 76.59M | 1.298 | 77.03M | nan | — | — |
| 1,000,000 | 13.944 | 71.71M | 13.155 | 76.02M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.303 ms**; native kernel **1.266 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.247 | 0.203 | 4.92M | nan | — | — |
| 100,000 | 10 | 1.491 | 0.930 | 10.75M | nan | — | — |
| 100,000 | 1,000 | 16.236 | 14.012 | 71.37M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 64.62M | 68.47M | 1.00× | 2.71M | 2.34M | 1.00× | — |
| 2 | 116.51M | 126.30M | 1.84× | 2.88M | 3.03M | 1.30× | — |
| 4 | 184.47M | 190.54M | 2.78× | 2.39M | 3.17M | 1.36× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
