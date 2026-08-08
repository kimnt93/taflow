# MathSinh benchmark (`SINH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.051 | 19.65M | 0.008 | 123.27M | 0.034 | 0.66× | 4.17× |
| 10,000 | 0.497 | 20.12M | 0.088 | 113.93M | 0.103 | 0.21× | 1.17× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.076 ms**; native kernel **0.012 ms**; TA-Lib 0.037 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.341 | 0.240 | 4.16M | 37.621 | 156.53× | 111.58× |
| 1,500 | 10 | 1.802 | 0.725 | 13.79M | 37.026 | 51.07× | 34.84× |
| 1,500 | 100 | 6.934 | 2.855 | 35.03M | 39.890 | 13.97× | 9.42× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
