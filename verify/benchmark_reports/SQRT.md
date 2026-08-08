# MathSqrt benchmark (`SQRT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 327.05M | 0.002 | 497.00M | 0.028 | 9.02× | 13.70× |
| 10,000 | 0.016 | 612.61M | 0.014 | 706.73M | 0.046 | 2.79× | 3.22× |
| 100,000 | 0.152 | 659.30M | 0.134 | 747.88M | 0.173 | 1.14× | 1.29× |
| 1,000,000 | 3.282 | 304.73M | 2.481 | 403.08M | 1.634 | 0.50× | 0.66× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.169 ms**; native kernel **0.133 ms**; TA-Lib 0.179 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.217 | 0.156 | 6.41M | 171.738 | 1100.33× | 177.58× |
| 100,000 | 10 | 0.932 | 0.594 | 16.84M | 168.660 | 284.01× | 44.60× |
| 100,000 | 1,000 | 4.270 | 2.880 | 347.26M | 167.124 | 58.03× | 10.49× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 246.99M | 359.00M | 1.00× | 2.86M | 3.49M | 1.00× | 330.44M |
| 2 | 421.96M | 660.70M | 1.84× | 3.13M | 3.55M | 1.02× | 381.95M |
| 4 | 442.22M | 813.89M | 2.27× | 2.97M | 3.53M | 1.01× | 367.70M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
