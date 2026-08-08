# TripleExponentialRateOfChange benchmark (`TRIX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 62.36M | 0.015 | 67.27M | 0.044 | 2.76× | 2.98× |
| 10,000 | 0.154 | 64.79M | 0.132 | 75.50M | 0.130 | 0.84× | 0.98× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.022 ms**; native kernel **0.021 ms**; TA-Lib 0.051 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.301 | 0.197 | 5.08M | 43.604 | 221.30× | 160.84× |
| 1,500 | 10 | 1.325 | 0.762 | 13.13M | 43.856 | 57.56× | 49.79× |
| 1,500 | 100 | 7.921 | 5.650 | 17.70M | 46.315 | 8.20× | 5.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
