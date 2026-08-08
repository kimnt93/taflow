# RollingCorrelation benchmark (`CORREL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 145.27M | 0.006 | 175.79M | 0.040 | 5.78× | 6.99× |
| 10,000 | 0.049 | 204.85M | 0.046 | 218.81M | 0.087 | 1.78× | 1.91× |
| 100,000 | 0.466 | 214.80M | 0.448 | 223.41M | 0.611 | 1.31× | 1.37× |
| 1,000,000 | 4.948 | 202.11M | 4.591 | 217.83M | 5.401 | 1.09× | 1.18× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.456 ms**; native kernel **0.442 ms**; TA-Lib 0.559 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.264 | 0.188 | 5.31M | 560.177 | 2977.16× | 194.41× |
| 100,000 | 10 | 1.566 | 0.886 | 11.29M | 543.437 | 613.50× | 37.52× |
| 100,000 | 1,000 | 7.524 | 6.065 | 164.88M | 557.724 | 91.96× | 6.58× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 129.22M | 181.08M | 1.00× | 2.67M | 2.95M | 1.00× | 118.09M |
| 2 | 252.93M | 371.61M | 2.05× | 2.68M | 3.07M | 1.04× | 143.70M |
| 4 | 292.44M | 598.57M | 3.31× | 2.62M | 2.80M | 0.95× | 142.26M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
