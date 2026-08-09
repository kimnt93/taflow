# SignedPower benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 52.75M | 0.018 | 55.15M | nan | — | — |
| 10,000 | 0.172 | 58.11M | 0.165 | 60.66M | nan | — | — |
| 100,000 | 1.626 | 61.51M | 1.618 | 61.82M | nan | — | — |
| 1,000,000 | 16.554 | 60.41M | 16.465 | 60.74M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.628 ms**; native kernel **1.604 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.237 | 0.165 | 6.07M | nan | — | — |
| 100,000 | 10 | 1.055 | 0.767 | 13.03M | nan | — | — |
| 100,000 | 1,000 | 18.285 | 17.475 | 57.22M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 52.73M | 53.69M | 1.00× | 2.65M | 3.35M | 1.00× | — |
| 2 | 100.26M | 104.81M | 1.95× | 2.73M | 3.26M | 0.97× | — |
| 4 | 150.30M | 202.22M | 3.77× | 2.92M | 3.14M | 0.94× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
