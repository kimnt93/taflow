# MovingAverageConvergenceDivergenceExtended benchmark (`MACDEXT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.221 | 4.53M | 0.025 | 40.57M | 0.059 | 0.27× | 2.38× |
| 10,000 | 2.198 | 4.55M | 0.224 | 44.74M | 0.123 | 0.06× | 0.55× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.330 ms**; native kernel **0.036 ms**; TA-Lib 0.063 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.601 | 0.222 | 4.50M | 61.128 | 275.18× | 233.28× |
| 1,500 | 10 | 4.112 | 1.141 | 8.76M | 64.301 | 56.34× | 45.33× |
| 1,500 | 100 | 18.468 | 4.677 | 21.38M | 60.562 | 12.95× | 11.42× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
