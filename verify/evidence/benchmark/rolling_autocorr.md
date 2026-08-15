# RollingAutocorr benchmark (`Autocorrelation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.060 | 16.72M | 0.059 | 17.00M | 0.252 | 4.21× | 4.28× |
| 10,000 | 0.602 | 16.61M | 0.588 | 17.01M | 1.048 | 1.74× | 1.78× |
| 100,000 | 5.907 | 16.93M | 5.897 | 16.96M | 9.103 | 1.54× | 1.54× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.144 | 0.298 | 2.07× |
| 1 | 5 | 0.304 | 1.415 | 4.65× |
| 1 | 10 | 0.428 | 2.635 | 6.16× |
| 10 | 1 | 0.047 | 0.231 | 4.88× |
| 10 | 5 | 0.186 | 1.415 | 7.59× |
| 10 | 10 | 0.407 | 2.409 | 5.93× |
| 100 | 1 | 0.048 | 0.238 | 4.96× |
| 100 | 5 | 0.200 | 1.459 | 7.30× |
| 100 | 10 | 0.429 | 2.757 | 6.42× |
| 1,000 | 1 | 0.106 | 0.321 | 3.02× |
| 1,000 | 5 | 0.217 | 1.902 | 8.78× |
| 1,000 | 10 | 0.477 | 3.415 | 7.17× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
