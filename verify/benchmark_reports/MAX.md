# RollingMax benchmark (`MAX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 224.66M | 0.003 | 294.66M | 0.034 | 7.67× | 10.06× |
| 10,000 | 0.033 | 300.72M | 0.032 | 314.24M | 0.079 | 2.38× | 2.49× |
| 100,000 | 0.356 | 280.99M | 0.334 | 299.47M | 0.498 | 1.40× | 1.49× |
| 1,000,000 | 4.068 | 245.83M | 3.584 | 279.02M | 4.733 | 1.16× | 1.32× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.354 ms**; native kernel **0.331 ms**; TA-Lib 0.504 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.236 | 0.167 | 5.99M | 502.053 | 3008.84× | 178.87× |
| 100,000 | 10 | 1.078 | 0.608 | 16.45M | 497.187 | 818.06× | 48.59× |
| 100,000 | 1,000 | 16.147 | 14.938 | 66.94M | 507.387 | 33.97× | 2.36× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 198.86M | 233.21M | 1.00× | 3.13M | 3.50M | 1.00× | 159.59M |
| 2 | 373.83M | 450.53M | 1.93× | 3.23M | 3.76M | 1.07× | 160.80M |
| 4 | 507.01M | 748.95M | 3.21× | 3.18M | 3.23M | 0.92× | 158.60M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
