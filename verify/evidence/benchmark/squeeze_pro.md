# SqueezePro benchmark (`squeeze_pro` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.206 | 4.85M | 0.201 | 4.98M | 7.838 | 38.04× | 39.07× |
| 10,000 | 1.817 | 5.50M | 1.844 | 5.42M | 11.387 | 6.27× | 6.18× |
| 100,000 | 18.676 | 5.35M | 18.532 | 5.40M | 48.122 | 2.58× | 2.60× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.122 | 0.383 | 3.13× |
| 1 | 5 | 0.498 | 1.759 | 3.53× |
| 1 | 10 | 0.744 | 3.443 | 4.63× |
| 10 | 1 | 0.079 | 0.347 | 4.36× |
| 10 | 5 | 0.364 | 1.738 | 4.78× |
| 10 | 10 | 0.740 | 3.456 | 4.67× |
| 100 | 1 | 0.113 | 8.210 | 72.59× |
| 100 | 5 | 0.615 | 43.272 | 70.41× |
| 100 | 10 | 0.745 | 87.343 | 117.19× |
| 1,000 | 1 | 0.288 | 8.838 | 30.70× |
| 1,000 | 5 | 0.549 | 46.264 | 84.32× |
| 1,000 | 10 | 0.885 | 95.033 | 107.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
