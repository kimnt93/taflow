# EmpiricalModeDecomposition benchmark (`EmpiricalModeDecomposition` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.037 | 27.33M | 0.035 | 28.64M | 0.229 | 6.26× | 6.56× |
| 10,000 | 0.343 | 29.16M | 0.332 | 30.08M | 0.886 | 2.58× | 2.66× |
| 100,000 | 3.414 | 29.30M | 3.527 | 28.36M | 7.019 | 2.06× | 1.99× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | 0.318 | 2.84× |
| 1 | 5 | 0.289 | 1.324 | 4.58× |
| 1 | 10 | 0.410 | 2.632 | 6.41× |
| 10 | 1 | 0.044 | 0.241 | 5.46× |
| 10 | 5 | 0.187 | 1.458 | 7.82× |
| 10 | 10 | 0.373 | 2.631 | 7.05× |
| 100 | 1 | 0.055 | 0.262 | 4.78× |
| 100 | 5 | 0.242 | 1.484 | 6.13× |
| 100 | 10 | 0.431 | 2.823 | 6.55× |
| 1,000 | 1 | 0.081 | 0.310 | 3.82× |
| 1,000 | 5 | 0.205 | 1.759 | 8.58× |
| 1,000 | 10 | 0.444 | 3.233 | 7.29× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
