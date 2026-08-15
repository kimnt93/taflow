# RollingPainIndex benchmark (`PainIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.042 | 23.92M | 0.040 | 24.82M | 0.181 | 4.32× | 4.48× |
| 10,000 | 0.407 | 24.57M | 0.392 | 25.49M | 0.687 | 1.69× | 1.75× |
| 100,000 | 4.059 | 24.64M | 3.888 | 25.72M | 5.683 | 1.40× | 1.46× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.075 | 0.236 | 3.13× |
| 1 | 5 | 0.222 | 1.013 | 4.56× |
| 1 | 10 | 0.417 | 2.096 | 5.03× |
| 10 | 1 | 0.044 | 0.195 | 4.41× |
| 10 | 5 | 0.181 | 0.967 | 5.34× |
| 10 | 10 | 0.386 | 2.165 | 5.61× |
| 100 | 1 | 0.048 | 0.194 | 4.05× |
| 100 | 5 | 0.233 | 1.034 | 4.44× |
| 100 | 10 | 0.454 | 2.159 | 4.75× |
| 1,000 | 1 | 0.092 | 0.270 | 2.94× |
| 1,000 | 5 | 0.213 | 1.258 | 5.90× |
| 1,000 | 10 | 0.435 | 2.772 | 6.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
