# RollingKellyCriterion benchmark (`KellyCriterion` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 44.13M | 0.022 | 46.24M | 0.071 | 3.13× | 3.28× |
| 10,000 | 0.182 | 54.98M | 0.181 | 55.10M | 0.551 | 3.03× | 3.03× |
| 100,000 | 1.944 | 51.45M | 1.812 | 55.19M | 5.541 | 2.85× | 3.06× |
| 1,000,000 | 19.162 | 52.19M | 18.386 | 54.39M | 54.514 | 2.84× | 2.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.121 | 0.102 | 0.85× |
| 1 | 5 | 0.251 | 0.330 | 1.31× |
| 1 | 10 | 0.473 | 0.736 | 1.56× |
| 10 | 1 | 0.052 | 0.071 | 1.39× |
| 10 | 5 | 0.217 | 0.317 | 1.46× |
| 10 | 10 | 0.461 | 0.687 | 1.49× |
| 100 | 1 | 0.053 | 0.071 | 1.36× |
| 100 | 5 | 0.341 | 0.475 | 1.39× |
| 100 | 10 | 0.542 | 0.807 | 1.49× |
| 1,000 | 1 | 0.086 | 0.132 | 1.53× |
| 1,000 | 5 | 0.241 | 0.636 | 2.64× |
| 1,000 | 10 | 0.523 | 1.305 | 2.49× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
