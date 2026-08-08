# MovingAverageConvergenceDivergence benchmark (`MACD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 151.57M | 0.005 | 203.17M | 0.052 | 7.81× | 10.46× |
| 10,000 | 0.049 | 204.25M | 0.039 | 258.36M | 0.137 | 2.81× | 3.55× |
| 100,000 | 0.446 | 224.20M | 0.382 | 261.46M | 0.986 | 2.21× | 2.58× |
| 1,000,000 | 16.588 | 60.29M | 4.039 | 247.60M | 18.391 | 1.11× | 4.55× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.448 ms**; native kernel **0.372 ms**; TA-Lib 0.990 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.335 | 0.256 | 3.91M | 948.957 | 3709.56× | 157.37× |
| 100,000 | 10 | 1.874 | 1.270 | 7.87M | 958.835 | 755.08× | 32.43× |
| 100,000 | 1,000 | 126.116 | 75.073 | 13.32M | 1013.497 | 13.50× | 0.68× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 137.25M | 187.70M | 1.00× | 1.82M | 1.81M | 1.00× | 75.88M |
| 2 | 188.09M | 349.26M | 1.86× | 1.51M | 1.58M | 0.87× | 77.16M |
| 4 | 269.52M | 574.24M | 3.06× | 1.29M | 1.33M | 0.74× | 81.02M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
