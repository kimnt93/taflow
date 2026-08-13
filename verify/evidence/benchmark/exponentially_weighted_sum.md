# ExponentiallyWeightedSum benchmark (`exponentially weighted sum` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.026 | 37.85M | 0.021 | 47.81M | 0.192 | 7.27× | 9.18× |
| 10,000 | 0.149 | 67.27M | 0.142 | 70.39M | 1.813 | 12.20× | 12.76× |
| 100,000 | 1.372 | 72.88M | 1.331 | 75.14M | 17.407 | 12.69× | 13.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.285 | 0.139 | 0.49× |
| 1 | 5 | 0.396 | 0.456 | 1.15× |
| 1 | 10 | 0.579 | 0.807 | 1.39× |
| 10 | 1 | 0.062 | 0.085 | 1.38× |
| 10 | 5 | 0.292 | 0.406 | 1.39× |
| 10 | 10 | 0.583 | 0.865 | 1.48× |
| 100 | 1 | 0.067 | 0.110 | 1.66× |
| 100 | 5 | 0.290 | 0.491 | 1.69× |
| 100 | 10 | 0.616 | 1.008 | 1.64× |
| 1,000 | 1 | 0.084 | 0.269 | 3.19× |
| 1,000 | 5 | 0.307 | 1.429 | 4.66× |
| 1,000 | 10 | 0.652 | 2.613 | 4.01× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
