# MovingAverageConvergenceDivergence benchmark (`MACD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.213 | 4.68M | 0.016 | 61.89M | 0.052 | 0.24× | 3.21× |
| 10,000 | 2.143 | 4.67M | 0.155 | 64.70M | 0.137 | 0.06× | 0.89× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.317 ms**; native kernel **0.024 ms**; TA-Lib 0.058 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.359 | 0.452 | 2.21M | 56.113 | 124.16× | 90.76× |
| 1,500 | 10 | 4.583 | 1.008 | 9.92M | 59.319 | 58.82× | 42.50× |
| 1,500 | 100 | 18.593 | 3.763 | 26.57M | 57.322 | 15.23× | 11.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
