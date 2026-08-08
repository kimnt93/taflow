# CandleIdenticalThreeCrows benchmark (`CDLIDENTICAL3CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.20M | 0.040 | 24.70M | 0.036 | 0.80× | 0.89× |
| 10,000 | 0.424 | 23.58M | 0.409 | 24.46M | 0.126 | 0.30× | 0.31× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.062 ms**; native kernel **0.062 ms**; TA-Lib 0.041 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.706 | 0.330 | 3.03M | 40.827 | 123.85× | 86.12× |
| 1,500 | 10 | 3.027 | 1.569 | 6.37M | 40.882 | 26.06× | 18.07× |
| 1,500 | 100 | 9.567 | 6.495 | 15.40M | 41.949 | 6.46× | 4.61× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
