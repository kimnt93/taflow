# MathCeil benchmark (`CEIL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 138.61M | 0.005 | 184.10M | 0.036 | 4.92× | 6.54× |
| 10,000 | 0.028 | 353.87M | 0.029 | 339.51M | 0.044 | 1.57× | 1.51× |
| 100,000 | 0.281 | 355.99M | 0.238 | 420.37M | 0.198 | 0.70× | 0.83× |
| 1,000,000 | 3.062 | 326.59M | 2.417 | 413.66M | 1.644 | 0.54× | 0.68× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.107 | 1.07× |
| 1 | 5 | 0.413 | 0.531 | 1.29× |
| 1 | 10 | 0.509 | 0.977 | 1.92× |
| 10 | 1 | 0.050 | 0.088 | 1.77× |
| 10 | 5 | 0.260 | 0.472 | 1.81× |
| 10 | 10 | 0.542 | 1.035 | 1.91× |
| 100 | 1 | 0.050 | 0.092 | 1.84× |
| 100 | 5 | 0.238 | 0.445 | 1.87× |
| 100 | 10 | 0.560 | 1.137 | 2.03× |
| 1,000 | 1 | 0.069 | 0.119 | 1.73× |
| 1,000 | 5 | 0.273 | 0.492 | 1.80× |
| 1,000 | 10 | 0.533 | 1.079 | 2.02× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
