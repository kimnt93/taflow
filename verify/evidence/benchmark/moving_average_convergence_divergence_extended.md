# MovingAverageConvergenceDivergenceExtended benchmark (`MACDEXT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 61.00M | 0.015 | 65.81M | 0.064 | 3.91× | 4.22× |
| 10,000 | 0.134 | 74.54M | 0.127 | 78.65M | 0.135 | 1.01× | 1.06× |
| 100,000 | 1.400 | 71.40M | 1.179 | 84.81M | 0.847 | 0.60× | 0.72× |
| 1,000,000 | 24.738 | 40.42M | 21.815 | 45.84M | 9.950 | 0.40× | 0.46× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.184 | 1.89× |
| 1 | 5 | 0.349 | 0.813 | 2.33× |
| 1 | 10 | 0.639 | 4.073 | 6.38× |
| 10 | 1 | 0.090 | 0.231 | 2.55× |
| 10 | 5 | 0.416 | 1.275 | 3.07× |
| 10 | 10 | 0.736 | 1.271 | 1.73× |
| 100 | 1 | 0.058 | 0.111 | 1.92× |
| 100 | 5 | 0.274 | 0.620 | 2.26× |
| 100 | 10 | 0.573 | 1.320 | 2.30× |
| 1,000 | 1 | 0.071 | 0.124 | 1.75× |
| 1,000 | 5 | 0.300 | 0.753 | 2.51× |
| 1,000 | 10 | 0.725 | 1.418 | 1.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
