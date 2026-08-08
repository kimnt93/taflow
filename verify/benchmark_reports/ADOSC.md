# AccumulationDistributionOscillator benchmark (`ADOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.051 | 19.52M | 0.009 | 109.41M | 0.038 | 0.74× | 4.16× |
| 10,000 | 0.484 | 20.64M | 0.073 | 136.83M | 0.060 | 0.12× | 0.82× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.073 ms**; native kernel **0.013 ms**; TA-Lib 0.039 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.460 | 0.317 | 3.16M | 39.225 | 123.93× | 106.63× |
| 1,500 | 10 | 3.125 | 1.270 | 7.87M | 38.565 | 30.37× | 25.88× |
| 1,500 | 100 | 10.469 | 3.574 | 27.98M | 37.568 | 10.51× | 9.08× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
