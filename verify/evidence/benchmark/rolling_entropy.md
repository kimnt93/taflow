# RollingEntropy benchmark (`rolling Shannon entropy` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.059 | 16.98M | 0.058 | 17.39M | 0.049 | 0.83× | 0.85× |
| 10,000 | 0.567 | 17.63M | 0.575 | 17.38M | 0.122 | 0.22× | 0.21× |
| 100,000 | 5.649 | 17.70M | 5.810 | 17.21M | 0.966 | 0.17× | 0.17× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.109 | 0.112 | 1.02× |
| 1 | 5 | 0.233 | 0.451 | 1.94× |
| 1 | 10 | 0.440 | 0.854 | 1.94× |
| 10 | 1 | 0.043 | 0.086 | 1.99× |
| 10 | 5 | 0.177 | 0.387 | 2.19× |
| 10 | 10 | 0.415 | 0.855 | 2.06× |
| 100 | 1 | 0.047 | 0.122 | 2.60× |
| 100 | 5 | 0.203 | 0.545 | 2.68× |
| 100 | 10 | 0.456 | 1.193 | 2.62× |
| 1,000 | 1 | 0.106 | 0.122 | 1.14× |
| 1,000 | 5 | 0.251 | 0.694 | 2.77× |
| 1,000 | 10 | 0.485 | 1.529 | 3.15× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
