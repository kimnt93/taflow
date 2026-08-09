# RollSpread benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.043 | 23.19M | 0.042 | 23.74M | nan | — | — |
| 10,000 | 0.427 | 23.40M | 0.418 | 23.92M | nan | — | — |
| 100,000 | 4.483 | 22.30M | 4.255 | 23.50M | nan | — | — |
| 1,000,000 | 42.089 | 23.76M | 42.043 | 23.78M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **4.194 ms**; native kernel **4.236 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.251 | 0.198 | 5.05M | nan | — | — |
| 100,000 | 10 | 1.508 | 1.097 | 9.12M | nan | — | — |
| 100,000 | 1,000 | 43.616 | 44.216 | 22.62M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 21.16M | 22.15M | 1.00× | 2.44M | 2.52M | 1.00× | — |
| 2 | 40.54M | 42.72M | 1.93× | 2.95M | 3.06M | 1.21× | — |
| 4 | 49.71M | 63.26M | 2.86× | 2.77M | 3.04M | 1.20× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
