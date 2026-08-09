# UlcerIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.041 | 24.18M | 0.042 | 23.92M | nan | — | — |
| 10,000 | 0.396 | 25.25M | 0.396 | 25.26M | nan | — | — |
| 100,000 | 3.977 | 25.15M | 3.957 | 25.27M | nan | — | — |
| 1,000,000 | 40.308 | 24.81M | 39.578 | 25.27M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **3.923 ms**; native kernel **3.928 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.253 | 0.203 | 4.94M | nan | — | — |
| 100,000 | 10 | 1.328 | 0.854 | 11.70M | nan | — | — |
| 100,000 | 1,000 | 43.524 | 40.590 | 24.64M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 23.29M | 23.62M | 1.00× | 2.42M | 2.69M | 1.00× | — |
| 2 | 41.69M | 46.72M | 1.98× | 2.87M | 3.19M | 1.19× | — |
| 4 | 81.56M | 88.88M | 3.76× | 2.57M | 2.90M | 1.08× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
