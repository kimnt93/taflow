# MathTan benchmark (`TAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.064 | 15.56M | 0.018 | 55.22M | 0.045 | 0.70× | 2.47× |
| 10,000 | 0.640 | 15.62M | 0.204 | 49.10M | 0.239 | 0.37× | 1.17× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.109 ms**; native kernel **0.028 ms**; TA-Lib 0.054 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.332 | 0.196 | 5.10M | 57.310 | 292.38× | 130.99× |
| 1,500 | 10 | 1.954 | 0.846 | 11.82M | 56.414 | 66.69× | 30.74× |
| 1,500 | 100 | 9.238 | 4.071 | 24.56M | 64.904 | 15.94× | 6.90× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
