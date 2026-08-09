# Sessions benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 68.06M | 0.012 | 80.29M | nan | — | — |
| 10,000 | 0.085 | 117.93M | 0.078 | 128.92M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.019 ms**; native kernel **0.016 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.476 | 0.406 | 2.46M | nan | — | — |
| 1,500 | 10 | 1.946 | 1.082 | 9.24M | nan | — | — |
| 1,500 | 100 | 3.618 | 3.412 | 29.31M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.94M | 13.86M | 1.00× | 693.58K | 958.42K | 1.00× | — |
| 2 | 15.98M | 16.66M | 1.20× | 1.07M | 1.12M | 1.17× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
