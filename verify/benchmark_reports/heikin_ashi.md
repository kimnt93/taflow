# HeikinAshi benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 53.25M | 0.017 | 59.22M | nan | — | — |
| 10,000 | 0.125 | 79.95M | 0.121 | 82.66M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.025 ms**; native kernel **0.023 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.492 | 0.417 | 2.40M | nan | — | — |
| 1,500 | 10 | 1.781 | 1.166 | 8.58M | nan | — | — |
| 1,500 | 100 | 3.655 | 2.883 | 34.68M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 12.90M | 11.61M | 1.00× | 1.07M | 883.22K | 1.00× | — |
| 2 | 14.91M | 15.44M | 1.33× | 1.03M | 1.26M | 1.42× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
