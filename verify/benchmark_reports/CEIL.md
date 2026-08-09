# MathCeil benchmark (`CEIL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 233.52M | 0.003 | 295.71M | 0.028 | 6.51× | 8.25× |
| 10,000 | 0.029 | 342.90M | 0.026 | 388.45M | 0.040 | 1.37× | 1.55× |
| 100,000 | 0.275 | 364.29M | 0.247 | 404.40M | 0.153 | 0.56× | 0.62× |
| 1,000,000 | 3.445 | 290.26M | 3.108 | 321.71M | 1.428 | 0.41× | 0.46× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.270 ms**; native kernel **0.251 ms**; TA-Lib 0.156 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.236 | 0.142 | 7.02M | 154.920 | 1087.91× | 182.08× |
| 100,000 | 10 | 0.843 | 0.498 | 20.07M | 150.010 | 301.08× | 52.23× |
| 100,000 | 1,000 | 5.318 | 3.891 | 256.98M | 157.119 | 40.38× | 7.50× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 238.29M | 291.14M | 1.00× | 3.47M | 3.30M | 1.00× | 399.24M |
| 2 | 442.46M | 529.70M | 1.82× | 3.34M | 3.75M | 1.14× | 388.97M |
| 4 | 413.23M | 721.48M | 2.48× | 3.07M | 3.57M | 1.08× | 418.24M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
