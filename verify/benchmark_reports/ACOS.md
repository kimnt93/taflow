# MathAcos benchmark (`ACOS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 91.83M | 0.010 | 101.36M | 0.035 | 3.22× | 3.55× |
| 10,000 | 0.084 | 119.18M | 0.080 | 125.37M | 0.133 | 1.59× | 1.67× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.014 ms**; native kernel **0.013 ms**; TA-Lib 0.036 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.298 | 0.173 | 5.77M | 35.864 | 206.88× | 145.64× |
| 1,500 | 10 | 1.212 | 0.649 | 15.40M | 35.446 | 54.58× | 42.44× |
| 1,500 | 100 | 3.745 | 2.376 | 42.08M | 40.848 | 17.19× | 10.87× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.99M | 14.37M | 1.00× | 975.12K | 1.45M | 1.00× | 9.55M |
| 2 | 15.18M | 18.71M | 1.30× | 1.49M | 1.76M | 1.21× | 9.73M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
