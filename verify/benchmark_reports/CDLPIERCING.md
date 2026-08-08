# CandlePiercing benchmark (`CDLPIERCING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.027 | 37.27M | 0.025 | 40.24M | 0.032 | 1.18× | 1.27× |
| 10,000 | 0.264 | 37.82M | 0.266 | 37.53M | 0.130 | 0.49× | 0.49× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.040 ms**; native kernel **0.039 ms**; TA-Lib 0.035 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.400 | 0.312 | 3.21M | 35.194 | 112.85× | 93.17× |
| 1,500 | 10 | 2.890 | 1.392 | 7.18M | 35.130 | 25.24× | 21.06× |
| 1,500 | 100 | 7.988 | 6.592 | 15.17M | 36.412 | 5.52× | 4.40× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
