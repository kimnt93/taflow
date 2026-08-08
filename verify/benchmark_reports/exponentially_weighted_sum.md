# ExponentiallyWeightedSum benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 173.38M | 0.005 | 210.08M | nan | — | — |
| 10,000 | 0.041 | 242.03M | 0.036 | 280.83M | nan | — | — |
| 100,000 | 0.369 | 271.06M | 0.339 | 294.92M | nan | — | — |
| 1,000,000 | 4.052 | 246.76M | 3.607 | 277.25M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.375 ms**; native kernel **0.338 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.233 | 0.146 | 6.83M | nan | — | — |
| 100,000 | 10 | 0.950 | 0.560 | 17.87M | nan | — | — |
| 100,000 | 1,000 | 5.420 | 4.485 | 222.94M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 197.64M | 203.04M | 1.00× | 2.92M | 3.47M | 1.00× | — |
| 2 | 330.39M | 461.35M | 2.27× | 3.01M | 3.91M | 1.13× | — |
| 4 | 495.27M | 764.20M | 3.76× | 3.11M | 3.52M | 1.01× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
