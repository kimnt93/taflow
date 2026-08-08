# MathAsin benchmark (`ASIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.34M | 0.008 | 129.71M | 0.034 | 0.69× | 4.41× |
| 10,000 | 0.483 | 20.70M | 0.074 | 135.35M | 0.099 | 0.21× | 1.34× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.071 ms**; native kernel **0.011 ms**; TA-Lib 0.038 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.325 | 0.302 | 3.31M | 37.193 | 123.25× | 88.89× |
| 1,500 | 10 | 1.690 | 0.732 | 13.66M | 36.748 | 50.22× | 36.40× |
| 1,500 | 100 | 7.489 | 2.981 | 33.55M | 37.203 | 12.48× | 9.63× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
