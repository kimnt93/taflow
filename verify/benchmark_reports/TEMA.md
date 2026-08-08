# TripleExponentialMovingAverage benchmark (`TEMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.053 | 18.77M | 0.012 | 81.54M | 0.040 | 0.74× | 3.23× |
| 10,000 | 0.523 | 19.13M | 0.115 | 86.90M | 0.122 | 0.23× | 1.06× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.078 ms**; native kernel **0.018 ms**; TA-Lib 0.044 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.312 | 0.572 | 1.75M | 44.586 | 77.94× | 54.66× |
| 1,500 | 10 | 1.781 | 2.623 | 3.81M | 46.709 | 17.81× | 12.71× |
| 1,500 | 100 | 7.554 | 3.267 | 30.61M | 45.872 | 14.04× | 10.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
