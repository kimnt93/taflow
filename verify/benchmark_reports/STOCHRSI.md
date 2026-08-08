# StochasticRelativeStrengthIndex benchmark (`STOCHRSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 46.75M | 0.020 | 49.41M | 0.054 | 2.50× | 2.65× |
| 10,000 | 0.202 | 49.60M | 0.190 | 52.60M | 0.204 | 1.01× | 1.08× |
| 100,000 | 1.930 | 51.82M | 2.692 | 37.15M | 1.678 | 0.87× | 0.62× |
| 1,000,000 | 29.186 | 34.26M | 27.597 | 36.24M | 17.323 | 0.59× | 0.63× |

## Warm-up

Construct + canonical extend over 100,000 bars: **2.707 ms**; native kernel **2.432 ms**; TA-Lib 1.703 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.378 | 0.322 | 3.11M | 1654.319 | 5138.80× | 142.47× |
| 100,000 | 10 | 1.762 | 1.733 | 5.77M | 1594.576 | 920.28× | 25.58× |
| 100,000 | 1,000 | 112.389 | 100.659 | 9.93M | 1682.124 | 16.71× | 0.57× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 46.62M | 51.18M | 1.00× | 1.64M | 1.97M | 1.00× | 52.93M |
| 2 | 81.74M | 89.80M | 1.75× | 1.84M | 2.04M | 1.04× | 52.32M |
| 4 | 111.28M | 141.79M | 2.77× | 1.78M | 2.04M | 1.04× | 52.82M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
