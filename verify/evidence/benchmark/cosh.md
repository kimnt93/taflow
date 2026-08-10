# MathCosh benchmark (`COSH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 91.14M | 0.010 | 103.35M | 0.040 | 3.67× | 4.16× |
| 10,000 | 0.076 | 131.78M | 0.068 | 146.28M | 0.097 | 1.28× | 1.42× |
| 100,000 | 0.711 | 140.70M | 0.656 | 152.43M | 0.725 | 1.02× | 1.10× |
| 1,000,000 | 7.390 | 135.31M | 7.033 | 142.18M | 6.789 | 0.92× | 0.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.122 | 0.167 | 1.37× |
| 1 | 5 | 0.307 | 0.507 | 1.65× |
| 1 | 10 | 0.481 | 1.109 | 2.31× |
| 10 | 1 | 0.056 | 0.095 | 1.69× |
| 10 | 5 | 0.249 | 0.480 | 1.93× |
| 10 | 10 | 0.848 | 1.208 | 1.42× |
| 100 | 1 | 0.062 | 0.102 | 1.65× |
| 100 | 5 | 0.275 | 0.495 | 1.80× |
| 100 | 10 | 0.570 | 1.197 | 2.10× |
| 1,000 | 1 | 0.062 | 0.107 | 1.74× |
| 1,000 | 5 | 0.325 | 0.547 | 1.68× |
| 1,000 | 10 | 0.666 | 1.226 | 1.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
