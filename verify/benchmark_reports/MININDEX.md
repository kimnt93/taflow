# RollingArgmin benchmark (`MININDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.055 | 18.14M | 0.012 | 86.60M | 0.038 | 0.68× | 3.26× |
| 10,000 | 0.594 | 16.85M | 0.157 | 63.72M | 0.096 | 0.16× | 0.61× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.082 ms**; native kernel **0.018 ms**; TA-Lib 0.039 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.313 | 0.199 | 5.03M | 40.943 | 206.12× | 155.61× |
| 1,500 | 10 | 2.160 | 0.977 | 10.24M | 38.806 | 39.73× | 28.83× |
| 1,500 | 100 | 10.034 | 3.831 | 26.10M | 39.150 | 10.22× | 7.63× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
