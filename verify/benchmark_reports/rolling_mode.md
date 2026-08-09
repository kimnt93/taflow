# RollingMode benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.101 | 9.87M | 0.099 | 10.14M | nan | — | — |
| 10,000 | 1.004 | 9.96M | 0.970 | 10.31M | nan | — | — |
| 100,000 | 9.958 | 10.04M | 9.578 | 10.44M | nan | — | — |
| 1,000,000 | 99.980 | 10.00M | 95.849 | 10.43M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **9.903 ms**; native kernel **9.709 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.320 | 0.247 | 4.04M | nan | — | — |
| 100,000 | 10 | 1.910 | 1.580 | 6.33M | nan | — | — |
| 100,000 | 1,000 | 101.634 | 102.734 | 9.73M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.72M | 9.49M | 1.00× | 2.27M | 2.48M | 1.00× | — |
| 2 | 18.36M | 18.93M | 1.99× | 2.45M | 2.34M | 0.95× | — |
| 4 | 35.54M | 34.38M | 3.62× | 2.23M | 2.40M | 0.97× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
