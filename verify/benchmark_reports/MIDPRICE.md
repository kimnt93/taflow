# RollingMidprice benchmark (`MIDPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.308 | 3.24M | 0.038 | 26.12M | 0.039 | 0.13× | 1.01× |
| 10,000 | 3.060 | 3.27M | 0.430 | 23.27M | 0.102 | 0.03× | 0.24× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.458 ms**; native kernel **0.057 ms**; TA-Lib 0.044 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.371 | 0.254 | 3.94M | 43.555 | 171.62× | 122.68× |
| 1,500 | 10 | 8.621 | 1.349 | 7.41M | 45.726 | 33.90× | 23.67× |
| 1,500 | 100 | 32.529 | 6.704 | 14.92M | 42.680 | 6.37× | 4.64× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
