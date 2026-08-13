# MovingAverageConvergenceDivergenceExtended benchmark (`MACDEXT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.069 | 14.47M | 0.062 | 16.25M | 0.053 | 0.77× | 0.87× |
| 10,000 | 0.507 | 19.73M | 0.488 | 20.50M | 0.115 | 0.23× | 0.24× |
| 100,000 | 5.871 | 17.03M | 4.813 | 20.78M | 1.284 | 0.22× | 0.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.124 | 0.129 | 1.04× |
| 1 | 5 | 0.532 | 0.564 | 1.06× |
| 1 | 10 | 0.799 | 1.150 | 1.44× |
| 10 | 1 | 0.080 | 0.115 | 1.44× |
| 10 | 5 | 0.384 | 0.521 | 1.36× |
| 10 | 10 | 0.775 | 1.110 | 1.43× |
| 100 | 1 | 0.092 | 0.108 | 1.17× |
| 100 | 5 | 0.376 | 0.546 | 1.45× |
| 100 | 10 | 0.817 | 1.100 | 1.35× |
| 1,000 | 1 | 0.142 | 0.125 | 0.87× |
| 1,000 | 5 | 0.392 | 0.573 | 1.46× |
| 1,000 | 10 | 0.822 | 1.179 | 1.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
