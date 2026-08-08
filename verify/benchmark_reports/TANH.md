# MathTanh benchmark (`TANH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.89M | 0.004 | 243.52M | 0.029 | 0.64× | 7.10× |
| 10,000 | 0.449 | 22.29M | 0.034 | 295.13M | 0.054 | 0.12× | 1.60× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.068 ms**; native kernel **0.006 ms**; TA-Lib 0.030 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.303 | 0.174 | 5.75M | 30.681 | 176.28× | 144.96× |
| 1,500 | 10 | 1.649 | 0.676 | 14.78M | 32.210 | 47.61× | 38.00× |
| 1,500 | 100 | 6.825 | 2.294 | 43.59M | 31.104 | 13.56× | 11.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
