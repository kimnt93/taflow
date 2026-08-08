# CandleTakuri benchmark (`CDLTAKURI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 77.13M | 0.011 | 87.66M | 0.037 | 2.87× | 3.26× |
| 10,000 | 0.116 | 86.30M | 0.113 | 88.15M | 0.113 | 0.97× | 0.99× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.018 ms**; native kernel **0.017 ms**; TA-Lib 0.040 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.374 | 0.292 | 3.42M | 42.476 | 145.45× | 100.66× |
| 1,500 | 10 | 2.680 | 1.262 | 7.93M | 46.035 | 36.49× | 24.37× |
| 1,500 | 100 | 6.430 | 3.897 | 25.66M | 43.565 | 11.18× | 7.65× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
