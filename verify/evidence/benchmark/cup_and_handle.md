# CupAndHandle benchmark (`CupAndHandle` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.058 | 17.13M | 0.049 | 20.42M | 0.216 | 3.70× | 4.42× |
| 10,000 | 0.394 | 25.37M | 0.369 | 27.07M | 1.271 | 3.22× | 3.44× |
| 100,000 | 3.631 | 27.54M | 3.544 | 28.22M | 12.009 | 3.31× | 3.39× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.131 | 0.199 | 1.53× |
| 1 | 5 | 0.374 | 0.823 | 2.20× |
| 1 | 10 | 0.665 | 1.647 | 2.48× |
| 10 | 1 | 0.070 | 0.173 | 2.48× |
| 10 | 5 | 0.329 | 1.083 | 3.29× |
| 10 | 10 | 0.640 | 1.676 | 2.62× |
| 100 | 1 | 0.077 | 0.182 | 2.36× |
| 100 | 5 | 0.309 | 1.131 | 3.66× |
| 100 | 10 | 0.676 | 1.766 | 2.61× |
| 1,000 | 1 | 0.116 | 0.291 | 2.51× |
| 1,000 | 5 | 0.322 | 1.736 | 5.39× |
| 1,000 | 10 | 0.698 | 2.934 | 4.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
