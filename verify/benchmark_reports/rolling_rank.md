# RollingRank benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 48.95M | 0.019 | 52.50M | nan | — | — |
| 10,000 | 0.167 | 59.92M | 0.163 | 61.40M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.028 ms**; native kernel **0.027 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.253 | 0.183 | 5.47M | nan | — | — |
| 1,500 | 10 | 1.094 | 0.700 | 14.29M | nan | — | — |
| 1,500 | 100 | 3.291 | 2.797 | 35.75M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.22M | 12.34M | 1.00× | 1.13M | 1.24M | 1.00× | — |
| 2 | 17.75M | 19.30M | 1.56× | 1.08M | 1.57M | 1.26× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
