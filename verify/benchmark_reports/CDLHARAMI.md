# CandleHarami benchmark (`CDLHARAMI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 145.82M | 0.005 | 206.95M | 0.036 | 5.20× | 7.38× |
| 10,000 | 0.061 | 165.18M | 0.057 | 176.39M | 0.145 | 2.40× | 2.56× |
| 100,000 | 0.972 | 102.89M | 0.960 | 104.13M | 1.181 | 1.22× | 1.23× |
| 1,000,000 | 10.231 | 97.75M | 9.858 | 101.44M | 11.897 | 1.16× | 1.21× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.971 ms**; native kernel **0.953 ms**; TA-Lib 1.185 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.358 | 0.298 | 3.36M | 1185.433 | 3982.33× | 96.95× |
| 100,000 | 10 | 2.879 | 1.462 | 6.84M | 1178.794 | 806.49× | 19.56× |
| 100,000 | 1,000 | 33.314 | 26.255 | 38.09M | 1222.046 | 46.55× | 1.33× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 84.11M | 91.82M | 1.00× | 2.20M | 2.20M | 1.00× | 72.18M |
| 2 | 161.84M | 166.57M | 1.81× | 2.24M | 2.55M | 1.16× | 73.16M |
| 4 | 277.60M | 339.68M | 3.70× | 2.14M | 2.24M | 1.02× | 77.24M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
