# TypicalPrice benchmark (`TYPPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 156.53M | 0.005 | 203.04M | 0.028 | 4.38× | 5.69× |
| 10,000 | 0.020 | 488.77M | 0.017 | 573.66M | 0.033 | 1.60× | 1.88× |
| 100,000 | 0.170 | 588.90M | 0.136 | 735.26M | 0.084 | 0.49× | 0.62× |
| 1,000,000 | 2.386 | 419.15M | 1.961 | 509.82M | 1.539 | 0.64× | 0.78× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.122 | 1.31× |
| 1 | 5 | 0.254 | 0.449 | 1.77× |
| 1 | 10 | 0.505 | 0.938 | 1.86× |
| 10 | 1 | 0.052 | 0.094 | 1.79× |
| 10 | 5 | 0.238 | 0.428 | 1.80× |
| 10 | 10 | 0.476 | 0.980 | 2.06× |
| 100 | 1 | 0.052 | 0.088 | 1.68× |
| 100 | 5 | 0.235 | 0.435 | 1.86× |
| 100 | 10 | 0.477 | 0.908 | 1.90× |
| 1,000 | 1 | 0.052 | 0.088 | 1.70× |
| 1,000 | 5 | 0.243 | 0.426 | 1.75× |
| 1,000 | 10 | 0.509 | 0.918 | 1.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
