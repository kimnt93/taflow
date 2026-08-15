# ParabolicSarExtended benchmark (`SAREXT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 106.48M | 0.008 | 118.72M | 0.084 | 8.97× | 10.00× |
| 10,000 | 0.099 | 100.62M | 0.083 | 120.16M | 0.092 | 0.93× | 1.11× |
| 100,000 | 0.983 | 101.70M | 0.923 | 108.37M | 0.671 | 0.68× | 0.73× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.059 | 0.116 | 1.96× |
| 1 | 5 | 0.205 | 0.591 | 2.89× |
| 1 | 10 | 0.462 | 1.174 | 2.54× |
| 10 | 1 | 0.046 | 0.111 | 2.40× |
| 10 | 5 | 0.199 | 0.589 | 2.96× |
| 10 | 10 | 0.430 | 1.360 | 3.17× |
| 100 | 1 | 0.052 | 0.129 | 2.49× |
| 100 | 5 | 0.246 | 0.637 | 2.59× |
| 100 | 10 | 0.458 | 1.188 | 2.59× |
| 1,000 | 1 | 0.053 | 0.118 | 2.21× |
| 1,000 | 5 | 0.195 | 0.570 | 2.92× |
| 1,000 | 10 | 0.411 | 1.203 | 2.93× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
