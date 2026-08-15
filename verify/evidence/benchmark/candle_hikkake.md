# CandleHikkake benchmark (`CDLHIKKAKE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 164.08M | 0.003 | 356.70M | 0.031 | 5.07× | 11.01× |
| 10,000 | 0.062 | 160.85M | 0.056 | 177.15M | 0.076 | 1.22× | 1.34× |
| 100,000 | 0.628 | 159.33M | 0.629 | 158.95M | 0.493 | 0.79× | 0.78× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.104 | 0.154 | 1.49× |
| 1 | 5 | 0.252 | 0.485 | 1.92× |
| 1 | 10 | 0.377 | 0.927 | 2.46× |
| 10 | 1 | 0.042 | 0.093 | 2.23× |
| 10 | 5 | 0.193 | 0.439 | 2.28× |
| 10 | 10 | 0.410 | 0.924 | 2.25× |
| 100 | 1 | 0.042 | 0.089 | 2.13× |
| 100 | 5 | 0.202 | 0.445 | 2.21× |
| 100 | 10 | 0.395 | 0.921 | 2.33× |
| 1,000 | 1 | 0.046 | 0.091 | 1.97× |
| 1,000 | 5 | 0.205 | 0.456 | 2.22× |
| 1,000 | 10 | 0.429 | 1.018 | 2.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
