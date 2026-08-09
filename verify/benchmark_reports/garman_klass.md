# GarmanKlass benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.025 | 40.19M | 0.022 | 44.88M | nan | — | — |
| 10,000 | 0.190 | 52.62M | 0.218 | 45.77M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.033 ms**; native kernel **0.034 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.383 | 0.293 | 3.41M | nan | — | — |
| 1,500 | 10 | 2.691 | 1.345 | 7.43M | nan | — | — |
| 1,500 | 100 | 5.256 | 3.618 | 27.64M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.68M | 11.66M | 1.00× | 989.25K | 1.26M | 1.00× | — |
| 2 | 13.71M | 16.42M | 1.41× | 1.35M | 1.40M | 1.11× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
