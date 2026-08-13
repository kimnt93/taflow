# Decycler benchmark (`Decycler` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.032 | 31.40M | 0.027 | 37.02M | 0.153 | 4.82× | 5.68× |
| 10,000 | 0.212 | 47.08M | 0.199 | 50.28M | 0.468 | 2.20× | 2.35× |
| 100,000 | 1.899 | 52.66M | 1.798 | 55.62M | 3.578 | 1.88× | 1.99× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.263 | 2.76× |
| 1 | 5 | 0.337 | 0.941 | 2.79× |
| 1 | 10 | 0.588 | 2.068 | 3.52× |
| 10 | 1 | 0.075 | 0.194 | 2.60× |
| 10 | 5 | 0.280 | 0.945 | 3.37× |
| 10 | 10 | 0.593 | 2.139 | 3.61× |
| 100 | 1 | 0.067 | 0.199 | 2.95× |
| 100 | 5 | 0.301 | 0.948 | 3.15× |
| 100 | 10 | 0.621 | 2.085 | 3.36× |
| 1,000 | 1 | 0.085 | 0.225 | 2.64× |
| 1,000 | 5 | 0.303 | 1.120 | 3.70× |
| 1,000 | 10 | 0.592 | 2.467 | 4.16× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
