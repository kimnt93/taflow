# CandleUpsideGapTwoCrows benchmark (`CDLUPSIDEGAP2CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.106 | 9.41M | 0.093 | 10.70M | 0.033 | 0.31× | 0.36× |
| 10,000 | 0.839 | 11.92M | 0.838 | 11.94M | 0.122 | 0.14× | 0.15× |
| 100,000 | 8.438 | 11.85M | 8.019 | 12.47M | 0.946 | 0.11× | 0.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.122 | 0.153 | 1.25× |
| 1 | 5 | 0.464 | 0.443 | 0.95× |
| 1 | 10 | 0.635 | 0.912 | 1.44× |
| 10 | 1 | 0.071 | 0.090 | 1.27× |
| 10 | 5 | 0.316 | 0.425 | 1.34× |
| 10 | 10 | 0.661 | 0.911 | 1.38× |
| 100 | 1 | 0.080 | 0.095 | 1.19× |
| 100 | 5 | 0.332 | 0.482 | 1.45× |
| 100 | 10 | 0.713 | 0.932 | 1.31× |
| 1,000 | 1 | 0.157 | 0.112 | 0.71× |
| 1,000 | 5 | 0.337 | 0.476 | 1.41× |
| 1,000 | 10 | 0.700 | 1.035 | 1.48× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
