# CandleStalledPattern benchmark (`CDLSTALLEDPATTERN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.056 | 17.71M | 0.056 | 17.72M | 0.041 | 0.73× | 0.73× |
| 10,000 | 0.544 | 18.38M | 0.563 | 17.75M | 0.163 | 0.30× | 0.29× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.083 ms**; native kernel **0.085 ms**; TA-Lib 0.049 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.653 | 0.349 | 2.87M | 49.082 | 140.77× | 82.03× |
| 1,500 | 10 | 3.111 | 1.637 | 6.11M | 47.696 | 29.13× | 17.60× |
| 1,500 | 100 | 10.898 | 23.916 | 4.18M | 48.502 | 2.03× | 1.28× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
