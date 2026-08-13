# CandleMatchingLow benchmark (`CDLMATCHINGLOW` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.086 | 11.69M | 0.072 | 13.93M | 0.031 | 0.36× | 0.43× |
| 10,000 | 0.658 | 15.21M | 0.639 | 15.64M | 0.087 | 0.13× | 0.14× |
| 100,000 | 6.680 | 14.97M | 6.079 | 16.45M | 0.624 | 0.09× | 0.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.250 | 0.124 | 0.50× |
| 1 | 5 | 0.372 | 0.470 | 1.26× |
| 1 | 10 | 0.743 | 1.009 | 1.36× |
| 10 | 1 | 0.077 | 0.090 | 1.18× |
| 10 | 5 | 0.307 | 0.431 | 1.41× |
| 10 | 10 | 0.626 | 0.927 | 1.48× |
| 100 | 1 | 0.073 | 0.091 | 1.25× |
| 100 | 5 | 0.312 | 0.428 | 1.37× |
| 100 | 10 | 0.673 | 0.964 | 1.43× |
| 1,000 | 1 | 0.142 | 0.091 | 0.64× |
| 1,000 | 5 | 0.314 | 0.479 | 1.53× |
| 1,000 | 10 | 0.687 | 0.995 | 1.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
