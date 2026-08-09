# IntradayMomentumIndex benchmark (`IMI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 73.28M | 0.013 | 75.53M | 0.082 | 5.99× | 6.17× |
| 10,000 | 0.130 | 77.07M | 0.130 | 76.79M | 0.608 | 4.68× | 4.67× |
| 100,000 | 1.252 | 79.90M | 1.238 | 80.77M | 5.758 | 4.60× | 4.65× |
| 1,000,000 | 12.834 | 77.92M | 12.320 | 81.17M | 57.383 | 4.47× | 4.66× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.254 ms**; native kernel **1.278 ms**; TA-Lib 5.674 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.232 | 0.183 | 5.47M | 5624.405 | 30783.76× | 163.05× |
| 100,000 | 10 | 0.867 | 0.780 | 12.82M | 5613.127 | 7194.62× | 38.07× |
| 100,000 | 1,000 | 14.315 | 14.292 | 69.97M | 5838.748 | 408.54× | 5.75× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 71.86M | 71.67M | 1.00× | 3.05M | 2.69M | 1.00× | 17.45M |
| 2 | 116.15M | 118.09M | 1.65× | 2.39M | 2.44M | 0.91× | 13.44M |
| 4 | 192.67M | 215.24M | 3.00× | 2.30M | 2.37M | 0.88× | 15.95M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
