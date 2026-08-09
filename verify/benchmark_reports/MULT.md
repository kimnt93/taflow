# MathMultiply benchmark (`MULT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 196.84M | 0.003 | 304.44M | 0.032 | 6.31× | 9.76× |
| 10,000 | 0.010 | 1.01G | 0.007 | 1.41G | 0.033 | 3.38× | 4.71× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.005 ms**; native kernel **0.004 ms**; TA-Lib 0.030 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.342 | 0.195 | 5.13M | 29.981 | 153.69× | 148.01× |
| 1,500 | 10 | 1.600 | 0.755 | 13.25M | 31.040 | 41.13× | 37.26× |
| 1,500 | 100 | 3.383 | 1.862 | 53.72M | 29.299 | 15.74× | 15.92× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.85M | 10.70M | 1.00× | 835.52K | 1.21M | 1.00× | 7.33M |
| 2 | 17.15M | 22.20M | 2.08× | 1.33M | 1.37M | 1.14× | 10.09M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
