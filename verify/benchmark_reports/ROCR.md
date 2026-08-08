# RateOfChangeRatio benchmark (`ROCR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 261.35M | 0.003 | 360.61M | 0.031 | 8.02× | 11.07× |
| 10,000 | 0.022 | 460.14M | 0.019 | 528.24M | 0.042 | 1.95× | 2.23× |
| 100,000 | 0.199 | 502.28M | 0.178 | 560.81M | 0.124 | 0.62× | 0.70× |
| 1,000,000 | 2.187 | 457.15M | 1.818 | 550.13M | 1.125 | 0.51× | 0.62× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.199 ms**; native kernel **0.176 ms**; TA-Lib 0.126 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.227 | 0.157 | 6.36M | 126.791 | 805.92× | 189.84× |
| 100,000 | 10 | 0.950 | 0.501 | 19.96M | 123.726 | 246.91× | 58.17× |
| 100,000 | 1,000 | 4.237 | 3.028 | 330.20M | 124.732 | 41.19× | 10.02× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 333.43M | 385.14M | 1.00× | 3.79M | 3.45M | 1.00× | 482.10M |
| 2 | 530.11M | 672.71M | 1.75× | 3.25M | 3.79M | 1.10× | 474.01M |
| 4 | 734.20M | 1.27G | 3.30× | 3.16M | 3.31M | 0.96× | 479.80M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
