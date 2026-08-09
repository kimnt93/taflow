# SqueezePro benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.051 | 19.45M | 0.061 | 16.35M | nan | — | — |
| 10,000 | 0.444 | 22.51M | 0.458 | 21.83M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.108 ms**; native kernel **0.079 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.402 | 0.326 | 3.07M | nan | — | — |
| 1,500 | 10 | 2.414 | 1.359 | 7.36M | nan | — | — |
| 1,500 | 100 | 7.190 | 5.709 | 17.52M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.94M | 8.13M | 1.00× | 1.04M | 1.09M | 1.00× | — |
| 2 | 10.38M | 14.30M | 1.76× | 1.19M | 1.02M | 0.94× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
