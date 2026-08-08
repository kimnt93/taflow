# RateOfChange benchmark (`ROC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 252.31M | 0.003 | 342.65M | 0.029 | 7.42× | 10.08× |
| 10,000 | 0.024 | 410.56M | 0.021 | 466.17M | 0.039 | 1.62× | 1.84× |
| 100,000 | 0.214 | 467.79M | 0.190 | 525.20M | 0.129 | 0.60× | 0.68× |
| 1,000,000 | 2.519 | 397.00M | 2.011 | 497.17M | 1.184 | 0.47× | 0.59× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.217 ms**; native kernel **0.190 ms**; TA-Lib 0.126 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.230 | 0.150 | 6.65M | 132.968 | 884.35× | 186.95× |
| 100,000 | 10 | 0.903 | 0.542 | 18.46M | 124.904 | 230.61× | 53.72× |
| 100,000 | 1,000 | 4.995 | 3.477 | 287.59M | 123.713 | 35.58× | 8.46× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 254.81M | 275.69M | 1.00× | 2.60M | 3.36M | 1.00× | 276.10M |
| 2 | 522.13M | 710.72M | 2.58× | 3.37M | 4.12M | 1.23× | 480.97M |
| 4 | 637.34M | 1.10G | 3.97× | 3.15M | 3.52M | 1.05× | 491.93M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
