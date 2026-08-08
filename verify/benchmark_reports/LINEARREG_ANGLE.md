# RollingLinearRegressionAngle benchmark (`LINEARREG_ANGLE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 42.26M | 0.022 | 44.97M | 0.050 | 2.12× | 2.26× |
| 10,000 | 0.223 | 44.79M | 0.223 | 44.79M | 0.251 | 1.12× | 1.12× |
| 100,000 | 2.648 | 37.77M | 2.201 | 45.42M | 2.243 | 0.85× | 1.02× |
| 1,000,000 | 23.307 | 42.91M | 22.810 | 43.84M | 22.087 | 0.95× | 0.97× |

## Warm-up

Construct + canonical extend over 100,000 bars: **2.202 ms**; native kernel **2.267 ms**; TA-Lib 2.236 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.257 | 0.212 | 4.72M | 2269.397 | 10716.73× | 134.21× |
| 100,000 | 10 | 1.245 | 0.925 | 10.81M | 2272.922 | 2457.66× | 33.89× |
| 100,000 | 1,000 | 34.550 | 25.195 | 39.69M | 2306.575 | 91.55× | 2.18× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 40.23M | 40.45M | 1.00× | 2.94M | 3.31M | 1.00× | 38.06M |
| 2 | 70.84M | 73.19M | 1.81× | 2.22M | 3.08M | 0.93× | 41.03M |
| 4 | 138.03M | 137.64M | 3.40× | 2.58M | 2.68M | 0.81× | 38.20M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
