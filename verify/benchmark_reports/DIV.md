# MathDivide benchmark (`DIV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 201.34M | 0.003 | 291.36M | 0.029 | 5.91× | 8.55× |
| 10,000 | 0.012 | 856.25M | 0.008 | 1.23G | 0.034 | 2.89× | 4.17× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.005 ms**; native kernel **0.004 ms**; TA-Lib 0.029 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.346 | 0.196 | 5.11M | 30.040 | 153.62× | 138.84× |
| 1,500 | 10 | 1.779 | 0.752 | 13.30M | 29.452 | 39.16× | 38.09× |
| 1,500 | 100 | 3.475 | 1.904 | 52.52M | 29.185 | 15.33× | 15.57× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 11.13M | 17.59M | 1.00× | 1.22M | 1.43M | 1.00× | 8.20M |
| 2 | 18.88M | 20.92M | 1.19× | 1.33M | 1.56M | 1.09× | 10.88M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
