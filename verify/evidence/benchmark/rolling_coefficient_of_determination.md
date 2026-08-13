# RollingCoefficientOfDetermination benchmark (`rolling squared correlation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.645 | 1.55M | 0.646 | 1.55M | 0.281 | 0.44× | 0.44× |
| 10,000 | 6.352 | 1.57M | 7.473 | 1.34M | 1.795 | 0.28× | 0.24× |
| 100,000 | 64.331 | 1.55M | 64.710 | 1.55M | 22.707 | 0.35× | 0.35× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.145 | 0.296 | 2.04× |
| 1 | 5 | 0.453 | 0.668 | 1.47× |
| 1 | 10 | 0.614 | 1.352 | 2.20× |
| 10 | 1 | 0.071 | 0.137 | 1.93× |
| 10 | 5 | 0.315 | 0.666 | 2.11× |
| 10 | 10 | 0.629 | 1.350 | 2.15× |
| 100 | 1 | 0.131 | 0.228 | 1.74× |
| 100 | 5 | 0.322 | 1.219 | 3.78× |
| 100 | 10 | 0.683 | 2.445 | 3.58× |
| 1,000 | 1 | 0.741 | 0.399 | 0.54× |
| 1,000 | 5 | 1.069 | 1.539 | 1.44× |
| 1,000 | 10 | 1.669 | 3.063 | 1.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
