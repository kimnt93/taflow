# ChaikinVolatility benchmark (`ChaikinVolatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 81.90M | 0.011 | 92.12M | 0.239 | 19.58× | 22.02× |
| 10,000 | 0.077 | 129.19M | 0.075 | 133.82M | 0.835 | 10.78× | 11.17× |
| 100,000 | 0.725 | 138.01M | 0.715 | 139.84M | 7.132 | 9.84× | 9.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.141 | 0.307 | 2.18× |
| 1 | 5 | 0.258 | 1.701 | 6.61× |
| 1 | 10 | 0.520 | 3.288 | 6.32× |
| 10 | 1 | 0.085 | 0.324 | 3.82× |
| 10 | 5 | 0.253 | 1.663 | 6.57× |
| 10 | 10 | 0.529 | 2.812 | 5.32× |
| 100 | 1 | 0.054 | 0.258 | 4.76× |
| 100 | 5 | 0.244 | 1.634 | 6.70× |
| 100 | 10 | 0.576 | 3.088 | 5.36× |
| 1,000 | 1 | 0.088 | 0.351 | 3.99× |
| 1,000 | 5 | 0.255 | 1.974 | 7.75× |
| 1,000 | 10 | 0.539 | 3.444 | 6.39× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
