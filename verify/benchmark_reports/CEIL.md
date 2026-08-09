# MathCeil benchmark (`CEIL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 169.19M | 0.006 | 162.20M | 0.029 | 4.84× | 4.64× |
| 10,000 | 0.030 | 330.68M | 0.028 | 361.68M | 0.040 | 1.32× | 1.44× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.007 ms**; native kernel **0.006 ms**; TA-Lib 0.029 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.279 | 0.297 | 3.36M | 29.909 | 100.60× | 89.77× |
| 1,500 | 10 | 1.122 | 0.587 | 17.05M | 28.780 | 49.06× | 44.22× |
| 1,500 | 100 | 2.875 | 1.869 | 53.50M | 28.830 | 15.42× | 14.33× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.87M | 12.91M | 1.00× | 1.20M | 1.36M | 1.00× | 8.57M |
| 2 | 15.39M | 22.52M | 1.74× | 1.48M | 1.65M | 1.21× | 10.74M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
