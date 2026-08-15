# SessionRange benchmark (`SessionRange` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 46.58M | 0.017 | 59.93M | 0.688 | 32.04× | 41.22× |
| 10,000 | 0.165 | 60.79M | 0.154 | 64.90M | 5.541 | 33.68× | 35.96× |
| 100,000 | 1.710 | 58.47M | 1.626 | 61.49M | 61.633 | 36.04× | 37.90× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.072 | 0.286 | 3.98× |
| 1 | 5 | 0.263 | 1.195 | 4.53× |
| 1 | 10 | 0.402 | 2.413 | 6.00× |
| 10 | 1 | 0.051 | 0.235 | 4.60× |
| 10 | 5 | 0.190 | 1.326 | 6.98× |
| 10 | 10 | 0.413 | 2.519 | 6.10× |
| 100 | 1 | 0.045 | 0.293 | 6.46× |
| 100 | 5 | 0.202 | 1.590 | 7.86× |
| 100 | 10 | 0.473 | 3.020 | 6.39× |
| 1,000 | 1 | 0.063 | 0.996 | 15.78× |
| 1,000 | 5 | 0.234 | 4.298 | 18.39× |
| 1,000 | 10 | 0.491 | 9.108 | 18.56× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
