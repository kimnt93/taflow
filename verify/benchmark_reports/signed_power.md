# SignedPower benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 178.07M | 0.004 | 227.18M | nan | — | — |
| 10,000 | 0.023 | 441.15M | 0.020 | 495.19M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.006 ms**; native kernel **0.005 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.249 | 0.167 | 5.99M | nan | — | — |
| 1,500 | 10 | 1.415 | 0.522 | 19.17M | nan | — | — |
| 1,500 | 100 | 1.880 | 1.282 | 77.98M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 12.52M | 14.08M | 1.00× | 873.71K | 1.03M | 1.00× | — |
| 2 | 15.08M | 18.79M | 1.33× | 1.60M | 1.69M | 1.64× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
