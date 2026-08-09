# MathTan benchmark (`TAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.79M | 0.019 | 52.41M | 0.044 | 2.19× | 2.30× |
| 10,000 | 0.202 | 49.54M | 0.199 | 50.14M | 0.225 | 1.11× | 1.13× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.031 ms**; native kernel **0.029 ms**; TA-Lib 0.054 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.315 | 0.189 | 5.29M | 53.692 | 284.05× | 140.75× |
| 1,500 | 10 | 1.315 | 0.790 | 12.66M | 57.338 | 72.61× | 33.24× |
| 1,500 | 100 | 4.825 | 3.648 | 27.41M | 55.647 | 15.25× | 7.78× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.11M | 13.65M | 1.00× | 854.31K | 1.31M | 1.00× | 8.17M |
| 2 | 17.53M | 20.27M | 1.48× | 1.33M | 1.43M | 1.10× | 8.48M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
