# TripleTopBottom benchmark (`TripleTopBottom` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 72.85M | 0.011 | 93.54M | 0.233 | 17.01× | 21.84× |
| 10,000 | 0.095 | 105.75M | 0.088 | 113.98M | 1.383 | 14.62× | 15.76× |
| 100,000 | 0.894 | 111.89M | 0.845 | 118.31M | 12.916 | 14.45× | 15.28× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.175 | 0.209 | 1.19× |
| 1 | 5 | 0.386 | 0.832 | 2.16× |
| 1 | 10 | 0.562 | 1.703 | 3.03× |
| 10 | 1 | 0.058 | 0.169 | 2.92× |
| 10 | 5 | 0.247 | 1.125 | 4.55× |
| 10 | 10 | 0.529 | 1.731 | 3.27× |
| 100 | 1 | 0.059 | 0.181 | 3.08× |
| 100 | 5 | 0.263 | 1.245 | 4.74× |
| 100 | 10 | 0.527 | 1.894 | 3.59× |
| 1,000 | 1 | 0.067 | 0.301 | 4.47× |
| 1,000 | 5 | 0.305 | 1.816 | 5.96× |
| 1,000 | 10 | 0.542 | 3.126 | 5.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
