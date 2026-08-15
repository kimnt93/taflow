# AbsolutePriceOscillator benchmark (`APO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 175.03M | 0.005 | 213.07M | 0.039 | 6.91× | 8.41× |
| 10,000 | 0.035 | 288.34M | 0.033 | 303.16M | 0.080 | 2.31× | 2.43× |
| 100,000 | 0.335 | 298.79M | 0.318 | 314.04M | 0.474 | 1.42× | 1.49× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.111 | 1.27× |
| 1 | 5 | 0.282 | 0.525 | 1.86× |
| 1 | 10 | 0.427 | 0.969 | 2.27× |
| 10 | 1 | 0.044 | 0.095 | 2.16× |
| 10 | 5 | 0.182 | 0.484 | 2.66× |
| 10 | 10 | 0.416 | 1.050 | 2.52× |
| 100 | 1 | 0.045 | 0.093 | 2.06× |
| 100 | 5 | 0.189 | 0.468 | 2.47× |
| 100 | 10 | 0.404 | 0.961 | 2.38× |
| 1,000 | 1 | 0.045 | 0.108 | 2.39× |
| 1,000 | 5 | 0.224 | 0.512 | 2.28× |
| 1,000 | 10 | 0.429 | 1.024 | 2.39× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
