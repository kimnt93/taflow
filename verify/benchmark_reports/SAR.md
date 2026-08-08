# ParabolicSar benchmark (`SAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.44M | 0.010 | 97.41M | 0.037 | 0.75× | 3.58× |
| 10,000 | 0.522 | 19.17M | 0.111 | 90.39M | 0.092 | 0.18× | 0.83× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.078 ms**; native kernel **0.015 ms**; TA-Lib 0.039 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.355 | 0.221 | 4.52M | 37.989 | 171.88× | 147.55× |
| 1,500 | 10 | 1.588 | 0.951 | 10.51M | 39.165 | 41.18× | 34.68× |
| 1,500 | 100 | 7.575 | 5.073 | 19.71M | 39.778 | 7.84× | 6.39× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
