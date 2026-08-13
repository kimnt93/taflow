# TwiggsMoneyFlow benchmark (`TwiggsMoneyFlow` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.077 | 13.01M | 0.069 | 14.48M | 0.253 | 3.30× | 3.67× |
| 10,000 | 0.588 | 17.00M | 0.557 | 17.94M | 1.397 | 2.37× | 2.51× |
| 100,000 | 5.427 | 18.43M | 5.269 | 18.98M | 12.701 | 2.34× | 2.41× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.235 | 2.39× |
| 1 | 5 | 0.457 | 1.091 | 2.38× |
| 1 | 10 | 0.653 | 2.442 | 3.74× |
| 10 | 1 | 0.082 | 0.218 | 2.67× |
| 10 | 5 | 0.329 | 1.076 | 3.27× |
| 10 | 10 | 0.924 | 2.856 | 3.09× |
| 100 | 1 | 0.093 | 0.226 | 2.43× |
| 100 | 5 | 0.356 | 1.401 | 3.93× |
| 100 | 10 | 0.665 | 2.412 | 3.63× |
| 1,000 | 1 | 0.134 | 0.350 | 2.61× |
| 1,000 | 5 | 0.317 | 1.906 | 6.01× |
| 1,000 | 10 | 0.697 | 3.602 | 5.17× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
