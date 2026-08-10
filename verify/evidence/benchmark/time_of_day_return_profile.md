# TimeOfDayReturnProfile benchmark (`TimeOfDayReturnProfile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.051 | 19.54M | 0.043 | 23.35M | 1.587 | 31.01× | 37.07× |
| 10,000 | 0.418 | 23.90M | 0.410 | 24.37M | 15.411 | 36.84× | 37.55× |
| 100,000 | 4.647 | 21.52M | 3.573 | 27.99M | 175.013 | 37.66× | 48.98× |
| 1,000,000 | 145.186 | 6.89M | 81.005 | 12.34M | 1882.500 | 12.97× | 23.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.075 | 0.330 | 4.37× |
| 1 | 5 | 0.351 | 1.524 | 4.34× |
| 1 | 10 | 0.592 | 2.804 | 4.73× |
| 10 | 1 | 0.061 | 0.291 | 4.76× |
| 10 | 5 | 0.280 | 1.613 | 5.76× |
| 10 | 10 | 0.603 | 3.119 | 5.17× |
| 100 | 1 | 0.066 | 0.423 | 6.42× |
| 100 | 5 | 0.284 | 2.261 | 7.97× |
| 100 | 10 | 0.618 | 4.349 | 7.04× |
| 1,000 | 1 | 0.097 | 2.010 | 20.75× |
| 1,000 | 5 | 0.303 | 9.338 | 30.81× |
| 1,000 | 10 | 0.750 | 20.197 | 26.92× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
