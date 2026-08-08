# MathAdd benchmark (`ADD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 407.75M | 0.001 | 973.40M | 0.030 | 12.05× | 28.76× |
| 10,000 | 0.008 | 1.26G | 0.004 | 2.51G | 0.034 | 4.25× | 8.44× |
| 100,000 | 0.061 | 1.64G | 0.039 | 2.55G | 0.069 | 1.13× | 1.75× |
| 1,000,000 | 1.382 | 723.47M | 0.894 | 1.12G | 1.003 | 0.73× | 1.12× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.063 ms**; native kernel **0.037 ms**; TA-Lib 0.069 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.291 | 0.196 | 5.09M | 70.480 | 358.85× | 147.19× |
| 100,000 | 10 | 1.568 | 0.786 | 12.73M | 67.452 | 85.83× | 38.07× |
| 100,000 | 1,000 | 3.648 | 2.198 | 455.05M | 69.155 | 31.47× | 13.96× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 509.64M | 661.35M | 1.00× | 2.97M | 2.63M | 1.00× | 597.69M |
| 2 | 759.09M | 1.20G | 1.82× | 2.71M | 3.45M | 1.31× | 540.08M |
| 4 | 748.66M | 1.34G | 2.02× | 2.37M | 3.06M | 1.16× | 531.59M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
