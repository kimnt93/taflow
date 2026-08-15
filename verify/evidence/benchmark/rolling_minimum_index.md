# RollingMinimumIndex benchmark (`MININDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 177.88M | 0.005 | 219.97M | 0.036 | 6.47× | 8.00× |
| 10,000 | 0.053 | 187.04M | 0.052 | 191.14M | 0.098 | 1.84× | 1.88× |
| 100,000 | 0.542 | 184.34M | 0.518 | 193.22M | 0.700 | 1.29× | 1.35× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.128 | 1.46× |
| 1 | 5 | 0.312 | 0.505 | 1.62× |
| 1 | 10 | 0.437 | 0.984 | 2.25× |
| 10 | 1 | 0.043 | 0.090 | 2.09× |
| 10 | 5 | 0.178 | 0.438 | 2.46× |
| 10 | 10 | 0.422 | 0.966 | 2.29× |
| 100 | 1 | 0.047 | 0.088 | 1.85× |
| 100 | 5 | 0.191 | 0.437 | 2.29× |
| 100 | 10 | 0.416 | 0.974 | 2.34× |
| 1,000 | 1 | 0.047 | 0.097 | 2.05× |
| 1,000 | 5 | 0.206 | 0.462 | 2.24× |
| 1,000 | 10 | 0.439 | 0.970 | 2.21× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
