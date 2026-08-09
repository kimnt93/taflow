# MathAdd benchmark (`ADD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 404.46M | 0.001 | 918.63M | 0.028 | 11.27× | 25.59× |
| 10,000 | 0.008 | 1.32G | 0.004 | 2.25G | 0.032 | 4.28× | 7.30× |
| 100,000 | 0.060 | 1.67G | 0.037 | 2.68G | 0.065 | 1.09× | 1.75× |
| 1,000,000 | 1.109 | 901.45M | 0.766 | 1.31G | 0.820 | 0.74× | 1.07× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.061 ms**; native kernel **0.037 ms**; TA-Lib 0.065 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.270 | 0.187 | 5.36M | 64.492 | 345.63× | 150.22× |
| 100,000 | 10 | 1.381 | 0.770 | 12.99M | 64.324 | 83.53× | 36.22× |
| 100,000 | 1,000 | 3.598 | 2.065 | 484.30M | 65.696 | 31.82× | 13.80× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 505.55M | 982.76M | 1.00× | 2.24M | 3.19M | 1.00× | 524.99M |
| 2 | 824.38M | 1.21G | 1.23× | 2.82M | 3.41M | 1.07× | 732.30M |
| 4 | 941.15M | 2.13G | 2.17× | 2.83M | 3.13M | 0.98× | 608.07M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
