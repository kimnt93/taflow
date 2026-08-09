# ParabolicSarExtended benchmark (`SAREXT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 70.07M | 0.014 | 70.31M | 0.058 | 4.04× | 4.05× |
| 10,000 | 0.119 | 84.04M | 0.109 | 91.67M | 0.100 | 0.84× | 0.92× |
| 100,000 | 1.295 | 77.23M | 1.150 | 86.96M | 0.662 | 0.51× | 0.58× |
| 1,000,000 | 11.745 | 85.14M | 11.829 | 84.54M | 6.503 | 0.55× | 0.55× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.122 | 0.215 | 1.76× |
| 1 | 5 | 0.288 | 0.628 | 2.18× |
| 1 | 10 | 0.519 | 1.369 | 2.64× |
| 10 | 1 | 0.048 | 0.122 | 2.52× |
| 10 | 5 | 0.224 | 0.598 | 2.67× |
| 10 | 10 | 0.538 | 1.383 | 2.57× |
| 100 | 1 | 0.059 | 0.126 | 2.13× |
| 100 | 5 | 0.258 | 0.586 | 2.27× |
| 100 | 10 | 0.596 | 1.382 | 2.32× |
| 1,000 | 1 | 0.064 | 0.123 | 1.93× |
| 1,000 | 5 | 0.276 | 0.653 | 2.37× |
| 1,000 | 10 | 0.587 | 1.424 | 2.42× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
