# RollingCorrelation benchmark (`CORREL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 107.07M | 0.008 | 126.47M | 0.040 | 4.26× | 5.03× |
| 10,000 | 0.051 | 197.91M | 0.049 | 204.73M | 0.090 | 1.78× | 1.84× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.012 ms**; native kernel **0.010 ms**; TA-Lib 0.042 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.355 | 0.205 | 4.88M | 41.531 | 202.55× | 162.52× |
| 1,500 | 10 | 1.727 | 0.868 | 11.53M | 42.232 | 48.68× | 39.84× |
| 1,500 | 100 | 3.963 | 2.476 | 40.39M | 42.432 | 17.14× | 15.08× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 12.41M | 14.99M | 1.00× | 1.22M | 1.50M | 1.00× | 9.13M |
| 2 | 19.38M | 21.76M | 1.45× | 1.36M | 1.49M | 1.00× | 8.76M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
