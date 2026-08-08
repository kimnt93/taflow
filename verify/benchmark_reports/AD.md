# AccumulationDistribution benchmark (`AD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.96M | 0.003 | 355.10M | 0.030 | 0.66× | 10.67× |
| 10,000 | 0.440 | 22.73M | 0.015 | 650.11M | 0.043 | 0.10× | 2.77× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.067 ms**; native kernel **0.004 ms**; TA-Lib 0.030 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.454 | 0.277 | 3.61M | 29.879 | 107.79× | 99.65× |
| 1,500 | 10 | 3.144 | 1.218 | 8.21M | 29.592 | 24.29× | 21.83× |
| 1,500 | 100 | 9.495 | 2.985 | 33.50M | 30.390 | 10.18× | 9.12× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
