# MovingAverageConvergenceDivergenceFixed benchmark (`MACDFIX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 180.33M | 0.004 | 238.04M | 0.046 | 8.37× | 11.05× |
| 10,000 | 0.031 | 322.08M | 0.025 | 393.49M | 0.130 | 4.19× | 5.12× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.007 ms**; native kernel **0.005 ms**; TA-Lib 0.051 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.285 | 0.210 | 4.76M | 49.509 | 235.65× | 171.59× |
| 1,500 | 10 | 1.216 | 0.645 | 15.50M | 54.513 | 84.48× | 59.43× |
| 1,500 | 100 | 3.512 | 2.566 | 38.97M | 52.598 | 20.50× | 15.35× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.74M | 12.78M | 1.00× | 1.20M | 1.46M | 1.00× | 7.41M |
| 2 | 15.72M | 21.44M | 1.68× | 1.40M | 1.57M | 1.08× | 8.91M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
