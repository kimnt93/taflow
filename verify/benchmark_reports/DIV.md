# MathDivide benchmark (`DIV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.277 | 3.61M | 0.001 | 703.23M | 0.029 | 0.10× | 20.19× |
| 10,000 | 2.582 | 3.87M | 0.006 | 1.65G | 0.035 | 0.01× | 5.70× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.384 ms**; native kernel **0.002 ms**; TA-Lib 0.031 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.337 | 0.204 | 4.91M | 28.607 | 140.38× | 134.19× |
| 1,500 | 10 | 4.727 | 0.885 | 11.29M | 29.237 | 33.02× | 30.31× |
| 1,500 | 100 | 27.854 | 2.318 | 43.14M | 29.423 | 12.69× | 11.94× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
