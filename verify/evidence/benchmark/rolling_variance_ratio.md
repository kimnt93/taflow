# RollingVarianceRatio benchmark (`VarianceRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.180 | 5.54M | 0.174 | 5.76M | 0.374 | 2.07× | 2.15× |
| 10,000 | 1.828 | 5.47M | 1.848 | 5.41M | 2.346 | 1.28× | 1.27× |
| 100,000 | 18.450 | 5.42M | 18.502 | 5.40M | 23.479 | 1.27× | 1.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.154 | 0.333 | 2.17× |
| 1 | 5 | 0.337 | 1.366 | 4.05× |
| 1 | 10 | 0.431 | 2.608 | 6.05× |
| 10 | 1 | 0.047 | 0.232 | 4.94× |
| 10 | 5 | 0.195 | 1.424 | 7.29× |
| 10 | 10 | 0.412 | 2.803 | 6.81× |
| 100 | 1 | 0.055 | 0.257 | 4.68× |
| 100 | 5 | 0.202 | 1.479 | 7.33× |
| 100 | 10 | 0.547 | 2.749 | 5.02× |
| 1,000 | 1 | 0.263 | 0.496 | 1.89× |
| 1,000 | 5 | 0.392 | 2.753 | 7.02× |
| 1,000 | 10 | 0.804 | 5.401 | 6.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
