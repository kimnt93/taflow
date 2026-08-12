# BalanceOfPower benchmark (`BOP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 116.29M | 0.008 | 132.10M | 0.029 | 3.33× | 3.79× |
| 10,000 | 0.033 | 298.78M | 0.031 | 321.60M | 0.041 | 1.22× | 1.31× |
| 100,000 | 0.289 | 345.67M | 0.270 | 370.33M | 0.132 | 0.46× | 0.49× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.122 | 0.114 | 0.93× |
| 1 | 5 | 0.298 | 0.441 | 1.48× |
| 1 | 10 | 0.499 | 0.967 | 1.94× |
| 10 | 1 | 0.074 | 0.105 | 1.41× |
| 10 | 5 | 0.262 | 0.443 | 1.69× |
| 10 | 10 | 0.522 | 0.919 | 1.76× |
| 100 | 1 | 0.050 | 0.098 | 1.97× |
| 100 | 5 | 0.245 | 0.462 | 1.89× |
| 100 | 10 | 0.490 | 0.882 | 1.80× |
| 1,000 | 1 | 0.051 | 0.082 | 1.62× |
| 1,000 | 5 | 0.256 | 0.423 | 1.65× |
| 1,000 | 10 | 0.528 | 0.898 | 1.70× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
