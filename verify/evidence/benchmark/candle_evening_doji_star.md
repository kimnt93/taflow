# CandleEveningDojiStar benchmark (`CDLEVENINGDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 147.22M | 0.004 | 259.70M | 0.036 | 5.32× | 9.38× |
| 10,000 | 0.076 | 131.97M | 0.074 | 135.34M | 0.117 | 1.55× | 1.58× |
| 100,000 | 0.817 | 122.45M | 0.804 | 124.33M | 0.826 | 1.01× | 1.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.124 | 0.219 | 1.76× |
| 1 | 5 | 0.292 | 0.470 | 1.61× |
| 1 | 10 | 0.375 | 0.916 | 2.44× |
| 10 | 1 | 0.042 | 0.087 | 2.08× |
| 10 | 5 | 0.183 | 0.430 | 2.35× |
| 10 | 10 | 0.394 | 0.951 | 2.41× |
| 100 | 1 | 0.046 | 0.094 | 2.03× |
| 100 | 5 | 0.187 | 0.449 | 2.41× |
| 100 | 10 | 0.399 | 0.995 | 2.49× |
| 1,000 | 1 | 0.053 | 0.101 | 1.90× |
| 1,000 | 5 | 0.195 | 0.527 | 2.71× |
| 1,000 | 10 | 0.417 | 1.028 | 2.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
