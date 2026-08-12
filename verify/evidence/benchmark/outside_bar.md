# OutsideBar benchmark (`outside bar relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 84.33M | 0.006 | 154.23M | 0.024 | 2.02× | 3.70× |
| 10,000 | 0.035 | 286.26M | 0.032 | 314.81M | 0.046 | 1.31× | 1.44× |
| 100,000 | 0.366 | 273.06M | 0.282 | 354.12M | 0.241 | 0.66× | 0.85× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.074 | 0.95× |
| 1 | 5 | 0.281 | 0.361 | 1.29× |
| 1 | 10 | 0.462 | 0.768 | 1.66× |
| 10 | 1 | 0.055 | 0.076 | 1.38× |
| 10 | 5 | 0.232 | 0.347 | 1.50× |
| 10 | 10 | 0.496 | 0.739 | 1.49× |
| 100 | 1 | 0.049 | 0.075 | 1.52× |
| 100 | 5 | 0.246 | 0.402 | 1.63× |
| 100 | 10 | 0.527 | 0.769 | 1.46× |
| 1,000 | 1 | 0.056 | 0.077 | 1.39× |
| 1,000 | 5 | 0.252 | 0.506 | 2.01× |
| 1,000 | 10 | 0.533 | 1.229 | 2.30× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
