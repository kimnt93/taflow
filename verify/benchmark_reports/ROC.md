# RateOfChange benchmark (`ROC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.74M | 0.004 | 238.30M | 0.031 | 0.68× | 7.41× |
| 10,000 | 0.449 | 22.27M | 0.037 | 273.59M | 0.049 | 0.11× | 1.33× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.073 ms**; native kernel **0.006 ms**; TA-Lib 0.034 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.296 | 0.181 | 5.51M | 34.542 | 190.38× | 170.94× |
| 1,500 | 10 | 3.090 | 1.211 | 8.26M | 31.537 | 26.04× | 25.69× |
| 1,500 | 100 | 7.111 | 2.431 | 41.14M | 33.983 | 13.98× | 12.15× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
