# MathLog1p benchmark (`numpy.log1p` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 107.58M | 0.009 | 113.51M | 0.021 | 2.22× | 2.35× |
| 10,000 | 0.078 | 128.23M | 0.080 | 125.78M | 0.096 | 1.22× | 1.20× |
| 100,000 | 0.822 | 121.70M | 0.745 | 134.26M | 0.762 | 0.93× | 1.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.135 | 0.108 | 0.80× |
| 1 | 5 | 0.206 | 0.272 | 1.32× |
| 1 | 10 | 0.372 | 0.637 | 1.71× |
| 10 | 1 | 0.040 | 0.063 | 1.57× |
| 10 | 5 | 0.179 | 0.261 | 1.45× |
| 10 | 10 | 0.401 | 0.561 | 1.40× |
| 100 | 1 | 0.041 | 0.061 | 1.46× |
| 100 | 5 | 0.200 | 0.306 | 1.53× |
| 100 | 10 | 0.394 | 0.615 | 1.56× |
| 1,000 | 1 | 0.049 | 0.071 | 1.46× |
| 1,000 | 5 | 0.195 | 0.347 | 1.78× |
| 1,000 | 10 | 0.430 | 0.817 | 1.90× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
