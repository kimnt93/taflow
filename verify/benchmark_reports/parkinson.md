# Parkinson benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 64.17M | 0.015 | 68.82M | nan | — | — |
| 10,000 | 0.131 | 76.57M | 0.126 | 79.55M | nan | — | — |
| 100,000 | 1.262 | 79.23M | 1.240 | 80.67M | nan | — | — |
| 1,000,000 | 13.127 | 76.18M | 12.666 | 78.95M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.259 ms**; native kernel **1.230 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.240 | 0.191 | 5.22M | nan | — | — |
| 100,000 | 10 | 1.583 | 0.790 | 12.66M | nan | — | — |
| 100,000 | 1,000 | 15.106 | 14.018 | 71.34M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 61.62M | 67.94M | 1.00× | 2.84M | 3.40M | 1.00× | — |
| 2 | 119.77M | 133.65M | 1.97× | 3.14M | 2.97M | 0.87× | — |
| 4 | 221.90M | 219.14M | 3.23× | 2.95M | 3.09M | 0.91× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
