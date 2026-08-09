# OrderBlock benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.085 | 11.70M | 0.084 | 11.84M | nan | — | — |
| 10,000 | 1.139 | 8.78M | 0.918 | 10.89M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.131 ms**; native kernel **0.130 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.679 | 0.537 | 1.86M | nan | — | — |
| 1,500 | 10 | 3.513 | 2.127 | 4.70M | nan | — | — |
| 1,500 | 100 | 11.393 | 10.005 | 10.00M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 5.25M | 6.19M | 1.00× | 968.82K | 808.58K | 1.00× | — |
| 2 | 10.41M | 10.17M | 1.64× | 991.42K | 887.52K | 1.10× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
