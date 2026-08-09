# MathCosh benchmark (`COSH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 103.56M | 0.009 | 116.74M | 0.037 | 3.87× | 4.36× |
| 10,000 | 0.078 | 128.59M | 0.084 | 118.70M | 0.084 | 1.08× | 0.99× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.013 ms**; native kernel **0.012 ms**; TA-Lib 0.035 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.295 | 0.171 | 5.84M | 40.760 | 238.02× | 153.47× |
| 1,500 | 10 | 1.194 | 0.641 | 15.59M | 37.826 | 58.97× | 41.38× |
| 1,500 | 100 | 3.404 | 2.268 | 44.09M | 35.858 | 15.81× | 11.77× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.95M | 14.60M | 1.00× | 1.33M | 1.67M | 1.00× | 9.85M |
| 2 | 15.33M | 20.12M | 1.38× | 1.28M | 1.70M | 1.02× | 8.95M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
