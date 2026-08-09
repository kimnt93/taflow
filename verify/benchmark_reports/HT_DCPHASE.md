# HilbertTransformDominantCyclePhase benchmark (`HT_DCPHASE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.093 | 10.73M | 0.092 | 10.87M | 0.420 | 4.50× | 4.56× |
| 10,000 | 1.000 | 10.00M | 0.976 | 10.25M | 4.210 | 4.21× | 4.31× |
| 100,000 | 9.748 | 10.26M | 9.680 | 10.33M | 41.208 | 4.23× | 4.26× |
| 1,000,000 | 97.891 | 10.22M | 97.075 | 10.30M | 415.445 | 4.24× | 4.28× |

## Warm-up

Construct + canonical extend over 100,000 bars: **9.699 ms**; native kernel **9.903 ms**; TA-Lib 41.287 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.312 | 0.248 | 4.03M | 42366.801 | 170588.01× | 130.94× |
| 100,000 | 10 | 1.982 | 1.585 | 6.31M | 41709.894 | 26312.73× | 22.61× |
| 100,000 | 1,000 | 106.570 | 98.925 | 10.11M | 41542.373 | 419.94× | 4.58× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.80M | 9.77M | 1.00× | 2.30M | 2.61M | 1.00× | 2.35M |
| 2 | 18.50M | 19.08M | 1.95× | 2.26M | 2.28M | 0.87× | 2.18M |
| 4 | 35.20M | 36.52M | 3.74× | 2.18M | 2.42M | 0.93× | 2.34M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
