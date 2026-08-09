# RateOfChangePercent benchmark (`ROCP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 274.86M | 0.003 | 372.15M | 0.029 | 8.10× | 10.97× |
| 10,000 | 0.021 | 471.76M | 0.018 | 542.40M | 0.039 | 1.84× | 2.11× |
| 100,000 | 0.192 | 520.08M | 0.173 | 578.79M | 0.122 | 0.63× | 0.70× |
| 1,000,000 | 2.116 | 472.58M | 1.796 | 556.70M | 1.067 | 0.50× | 0.59× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.192 ms**; native kernel **0.169 ms**; TA-Lib 0.123 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.225 | 0.143 | 7.00M | 121.024 | 847.65× | 194.89× |
| 100,000 | 10 | 0.864 | 0.475 | 21.03M | 122.050 | 256.69× | 58.82× |
| 100,000 | 1,000 | 4.202 | 3.058 | 326.97M | 121.999 | 39.89× | 9.65× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 331.16M | 382.50M | 1.00× | 3.40M | 3.86M | 1.00× | 514.31M |
| 2 | 567.79M | 544.58M | 1.42× | 2.99M | 4.06M | 1.05× | 502.24M |
| 4 | 627.05M | 974.71M | 2.55× | 3.11M | 3.51M | 0.91× | 455.36M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
