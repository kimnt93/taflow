# CandleSpinningTop benchmark (`CDLSPINNINGTOP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 107.30M | 0.007 | 134.25M | 0.032 | 3.42× | 4.28× |
| 10,000 | 0.130 | 77.17M | 0.125 | 80.04M | 0.126 | 0.97× | 1.01× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.013 ms**; native kernel **0.011 ms**; TA-Lib 0.035 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.385 | 0.293 | 3.42M | 33.479 | 114.45× | 101.23× |
| 1,500 | 10 | 2.631 | 1.197 | 8.36M | 34.177 | 28.56× | 24.44× |
| 1,500 | 100 | 6.384 | 3.850 | 25.97M | 33.909 | 8.81× | 7.63× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
