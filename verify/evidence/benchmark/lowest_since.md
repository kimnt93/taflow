# LowestSince benchmark (`lowest since condition` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 169.74M | 0.005 | 217.77M | 0.295 | 50.15× | 64.34× |
| 10,000 | 0.039 | 258.76M | 0.035 | 281.98M | 2.815 | 72.84× | 79.38× |
| 100,000 | 0.344 | 290.80M | 0.315 | 317.30M | 28.904 | 84.05× | 91.71× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.071 | 0.090 | 1.26× |
| 1 | 5 | 0.197 | 0.314 | 1.59× |
| 1 | 10 | 0.392 | 0.642 | 1.64× |
| 10 | 1 | 0.038 | 0.072 | 1.91× |
| 10 | 5 | 0.173 | 0.351 | 2.03× |
| 10 | 10 | 0.412 | 0.713 | 1.73× |
| 100 | 1 | 0.046 | 0.095 | 2.08× |
| 100 | 5 | 0.181 | 0.449 | 2.47× |
| 100 | 10 | 0.415 | 0.965 | 2.33× |
| 1,000 | 1 | 0.052 | 0.357 | 6.86× |
| 1,000 | 5 | 0.202 | 1.718 | 8.51× |
| 1,000 | 10 | 0.423 | 3.492 | 8.25× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
