# HilbertTransformTrendMode benchmark (`HT_TRENDMODE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.178 | 5.60M | 0.176 | 5.68M | 0.488 | 2.73× | 2.77× |
| 10,000 | 1.769 | 5.65M | 1.733 | 5.77M | 4.878 | 2.76× | 2.82× |
| 100,000 | 17.062 | 5.86M | 17.288 | 5.78M | 48.041 | 2.82× | 2.78× |
| 1,000,000 | 173.727 | 5.76M | 186.982 | 5.35M | 475.742 | 2.74× | 2.54× |

## Warm-up

Construct + canonical extend over 100,000 bars: **17.550 ms**; native kernel **17.612 ms**; TA-Lib 47.924 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.404 | 0.360 | 2.77M | 46406.907 | 128763.88× | 97.24× |
| 100,000 | 10 | 2.917 | 2.481 | 4.03M | 47054.523 | 18964.83× | 15.73× |
| 100,000 | 1,000 | 180.067 | 173.694 | 5.76M | 48936.402 | 281.74× | 2.94× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 5.66M | 5.57M | 1.00× | 1.63M | 1.88M | 1.00× | 2.10M |
| 2 | 10.81M | 10.85M | 1.95× | 1.77M | 1.85M | 0.98× | 2.07M |
| 4 | 17.91M | 20.06M | 3.60× | 1.72M | 1.97M | 1.05× | 1.87M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
