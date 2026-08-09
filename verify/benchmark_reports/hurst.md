# Hurst benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.066 | 15.16M | 0.066 | 15.26M | nan | — | — |
| 10,000 | 0.770 | 13.00M | 0.756 | 13.23M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.092 ms**; native kernel **0.092 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.308 | 0.236 | 4.24M | nan | — | — |
| 1,500 | 10 | 1.561 | 1.153 | 8.68M | nan | — | — |
| 1,500 | 100 | 7.430 | 7.073 | 14.14M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.60M | 7.46M | 1.00× | 972.73K | 1.25M | 1.00× | — |
| 2 | 11.58M | 13.61M | 1.83× | 1.32M | 1.37M | 1.10× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
