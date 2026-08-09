# TripleExponentialMovingAverage benchmark (`TEMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 75.33M | 0.012 | 82.12M | 0.050 | 3.74× | 4.07× |
| 10,000 | 0.098 | 101.95M | 0.097 | 103.35M | 0.117 | 1.19× | 1.21× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.017 ms**; native kernel **0.016 ms**; TA-Lib 0.043 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.294 | 0.220 | 4.54M | 43.385 | 197.08× | 142.20× |
| 1,500 | 10 | 1.167 | 0.667 | 15.00M | 44.124 | 66.18× | 46.59× |
| 1,500 | 100 | 3.634 | 2.496 | 40.06M | 47.431 | 19.00× | 13.02× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 11.18M | 10.58M | 1.00× | 1.31M | 1.47M | 1.00× | 8.53M |
| 2 | 18.61M | 21.78M | 2.06× | 1.41M | 1.62M | 1.10× | 9.47M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
