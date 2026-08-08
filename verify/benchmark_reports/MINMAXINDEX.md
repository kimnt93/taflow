# RollingMinMaxIndex benchmark (`MINMAXINDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.352 | 2.84M | 0.014 | 69.82M | 0.045 | 0.13× | 3.14× |
| 10,000 | 3.487 | 2.87M | 0.181 | 55.27M | 0.152 | 0.04× | 0.84× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.497 ms**; native kernel **0.021 ms**; TA-Lib 0.056 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.685 | 0.416 | 2.41M | 50.287 | 120.99× | 79.25× |
| 1,500 | 10 | 4.721 | 1.010 | 9.90M | 50.026 | 49.55× | 36.22× |
| 1,500 | 100 | 37.214 | 4.872 | 20.53M | 51.232 | 10.52× | 7.02× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
