# RollingCointegration benchmark (`Cointegration` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 7.106 | 140.73K | 6.874 | 145.47K | 3.076 | 0.43× | 0.45× |
| 10,000 | 72.923 | 137.13K | 78.608 | 127.21K | 29.834 | 0.41× | 0.38× |
| 100,000 | 731.690 | 136.67K | 716.895 | 139.49K | 305.755 | 0.42× | 0.43× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.293 | 2.97× |
| 1 | 5 | 0.417 | 1.370 | 3.29× |
| 1 | 10 | 0.667 | 2.509 | 3.76× |
| 10 | 1 | 0.074 | 0.249 | 3.38× |
| 10 | 5 | 0.323 | 1.351 | 4.18× |
| 10 | 10 | 0.673 | 2.622 | 3.90× |
| 100 | 1 | 0.621 | 0.492 | 0.79× |
| 100 | 5 | 0.921 | 2.610 | 2.84× |
| 100 | 10 | 1.456 | 5.240 | 3.60× |
| 1,000 | 1 | 7.149 | 3.601 | 0.50× |
| 1,000 | 5 | 9.750 | 17.424 | 1.79× |
| 1,000 | 10 | 14.712 | 36.725 | 2.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
