# TrianglePattern benchmark (`Triangle` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 58.73M | 0.014 | 73.19M | 0.251 | 14.75× | 18.38× |
| 10,000 | 0.109 | 91.95M | 0.117 | 85.26M | 1.469 | 13.50× | 12.52× |
| 100,000 | 1.334 | 74.95M | 1.058 | 94.49M | 12.910 | 9.68× | 12.20× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.220 | 2.37× |
| 1 | 5 | 0.346 | 0.853 | 2.47× |
| 1 | 10 | 0.553 | 1.774 | 3.21× |
| 10 | 1 | 0.062 | 0.173 | 2.78× |
| 10 | 5 | 0.262 | 1.095 | 4.18× |
| 10 | 10 | 0.533 | 1.791 | 3.36× |
| 100 | 1 | 0.063 | 0.185 | 2.93× |
| 100 | 5 | 0.268 | 1.216 | 4.54× |
| 100 | 10 | 0.562 | 2.054 | 3.66× |
| 1,000 | 1 | 0.072 | 0.306 | 4.27× |
| 1,000 | 5 | 0.281 | 1.904 | 6.78× |
| 1,000 | 10 | 0.573 | 3.098 | 5.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
