# CandleThreeStarsInSouth benchmark (`CDL3STARSINSOUTH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 69.05M | 0.014 | 70.09M | 0.036 | 2.49× | 2.52× |
| 10,000 | 0.074 | 135.71M | 0.067 | 148.61M | 0.134 | 1.82× | 1.99× |
| 100,000 | 0.766 | 130.47M | 0.789 | 126.71M | 1.011 | 1.32× | 1.28× |
| 1,000,000 | 8.195 | 122.02M | 8.866 | 112.79M | 9.701 | 1.18× | 1.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.121 | 0.179 | 1.48× |
| 1 | 5 | 0.290 | 0.470 | 1.62× |
| 1 | 10 | 0.601 | 1.049 | 1.74× |
| 10 | 1 | 0.062 | 0.108 | 1.73× |
| 10 | 5 | 0.275 | 0.502 | 1.82× |
| 10 | 10 | 0.665 | 1.175 | 1.77× |
| 100 | 1 | 0.063 | 0.095 | 1.50× |
| 100 | 5 | 0.271 | 0.441 | 1.63× |
| 100 | 10 | 0.575 | 1.092 | 1.90× |
| 1,000 | 1 | 0.076 | 0.120 | 1.58× |
| 1,000 | 5 | 0.335 | 0.515 | 1.54× |
| 1,000 | 10 | 0.624 | 1.223 | 1.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
