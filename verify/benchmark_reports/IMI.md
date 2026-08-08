# IntradayMomentumIndex benchmark (`IMI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.056 | 17.78M | 0.015 | 65.28M | 0.084 | 1.49× | 5.48× |
| 10,000 | 0.606 | 16.50M | 0.143 | 69.79M | 0.681 | 1.12× | 4.75× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.084 ms**; native kernel **0.021 ms**; TA-Lib 0.124 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.335 | 0.377 | 2.65M | 118.707 | 315.15× | 81.41× |
| 1,500 | 10 | 1.652 | 0.951 | 10.51M | 123.412 | 129.72× | 32.65× |
| 1,500 | 100 | 8.859 | 3.680 | 27.17M | 134.761 | 36.62× | 9.02× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
