# RateOfChange benchmark (`ROC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 170.89M | 0.005 | 189.59M | 0.042 | 7.14× | 7.92× |
| 10,000 | 0.025 | 400.62M | 0.023 | 435.52M | 0.051 | 2.05× | 2.23× |
| 100,000 | 0.230 | 434.39M | 0.206 | 486.60M | 0.167 | 0.73× | 0.81× |
| 1,000,000 | 2.808 | 356.10M | 2.307 | 433.41M | 1.546 | 0.55× | 0.67× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.146 | 1.77× |
| 1 | 5 | 0.291 | 0.525 | 1.80× |
| 1 | 10 | 0.591 | 1.522 | 2.58× |
| 10 | 1 | 0.071 | 0.125 | 1.76× |
| 10 | 5 | 0.285 | 0.623 | 2.19× |
| 10 | 10 | 0.944 | 1.188 | 1.26× |
| 100 | 1 | 0.065 | 0.106 | 1.64× |
| 100 | 5 | 0.309 | 0.576 | 1.87× |
| 100 | 10 | 0.627 | 1.118 | 1.78× |
| 1,000 | 1 | 0.068 | 0.124 | 1.81× |
| 1,000 | 5 | 0.277 | 0.533 | 1.93× |
| 1,000 | 10 | 0.558 | 1.160 | 2.08× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
