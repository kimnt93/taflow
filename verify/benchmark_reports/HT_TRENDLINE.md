# HilbertTransformTrendline benchmark (`HT_TRENDLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.072 | 13.80M | 0.070 | 14.20M | 0.081 | 1.12× | 1.15× |
| 10,000 | 0.725 | 13.78M | 0.722 | 13.84M | 0.615 | 0.85× | 0.85× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.107 ms**; native kernel **0.108 ms**; TA-Lib 0.114 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.312 | 0.244 | 4.10M | 111.083 | 455.67× | 120.35× |
| 1,500 | 10 | 1.430 | 1.343 | 7.45M | 114.129 | 84.99× | 22.54× |
| 1,500 | 100 | 10.386 | 9.021 | 11.09M | 120.177 | 13.32× | 4.09× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 5.14M | 6.25M | 1.00× | 770.94K | 1.35M | 1.00× | 5.74M |
| 2 | 8.21M | 9.64M | 1.54× | 1.41M | 1.55M | 1.15× | 6.50M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
