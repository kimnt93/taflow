# Crossover benchmark (`causal crossover` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.037 | 27.10M | 0.031 | 32.39M | 0.017 | 0.47× | 0.56× |
| 10,000 | 0.263 | 38.03M | 0.329 | 30.41M | 0.033 | 0.13× | 0.10× |
| 100,000 | 2.217 | 45.10M | 2.169 | 46.11M | 0.143 | 0.06× | 0.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.079 | 0.79× |
| 1 | 5 | 0.422 | 0.357 | 0.85× |
| 1 | 10 | 0.577 | 0.673 | 1.17× |
| 10 | 1 | 0.066 | 0.070 | 1.06× |
| 10 | 5 | 0.290 | 0.310 | 1.07× |
| 10 | 10 | 0.606 | 0.658 | 1.08× |
| 100 | 1 | 0.066 | 0.065 | 0.99× |
| 100 | 5 | 0.288 | 0.306 | 1.07× |
| 100 | 10 | 0.593 | 0.667 | 1.12× |
| 1,000 | 1 | 0.085 | 0.065 | 0.76× |
| 1,000 | 5 | 0.303 | 0.609 | 2.01× |
| 1,000 | 10 | 0.634 | 0.910 | 1.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
