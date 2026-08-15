# DoubleBollingerBands benchmark (`DoubleBollinger` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.42M | 0.043 | 23.31M | 0.645 | 14.46× | 15.03× |
| 10,000 | 0.413 | 24.19M | 0.393 | 25.44M | 4.504 | 10.90× | 11.46× |
| 100,000 | 3.988 | 25.07M | 4.059 | 24.64M | 52.167 | 13.08× | 12.85× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.293 | 2.97× |
| 1 | 5 | 0.248 | 1.444 | 5.83× |
| 1 | 10 | 0.412 | 2.795 | 6.78× |
| 10 | 1 | 0.048 | 0.262 | 5.50× |
| 10 | 5 | 0.223 | 1.496 | 6.72× |
| 10 | 10 | 0.403 | 3.018 | 7.48× |
| 100 | 1 | 0.051 | 0.308 | 6.08× |
| 100 | 5 | 0.212 | 1.543 | 7.27× |
| 100 | 10 | 0.417 | 3.421 | 8.21× |
| 1,000 | 1 | 0.094 | 0.871 | 9.27× |
| 1,000 | 5 | 0.238 | 3.953 | 16.60× |
| 1,000 | 10 | 0.492 | 8.011 | 16.28× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
