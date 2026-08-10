# DayOfWeekReturnProfile benchmark (`DayOfWeekProfile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.043 | 23.34M | 0.037 | 27.39M | 0.886 | 20.68× | 24.27× |
| 10,000 | 0.304 | 32.86M | 0.281 | 35.55M | 6.908 | 22.70× | 24.56× |
| 100,000 | 3.244 | 30.83M | 2.785 | 35.91M | 76.010 | 23.43× | 27.29× |
| 1,000,000 | 65.310 | 15.31M | 46.290 | 21.60M | 884.473 | 13.54× | 19.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.141 | 0.280 | 1.98× |
| 1 | 5 | 0.412 | 1.296 | 3.15× |
| 1 | 10 | 0.575 | 2.532 | 4.41× |
| 10 | 1 | 0.071 | 0.239 | 3.36× |
| 10 | 5 | 0.267 | 1.368 | 5.12× |
| 10 | 10 | 0.601 | 2.640 | 4.40× |
| 100 | 1 | 0.067 | 0.322 | 4.80× |
| 100 | 5 | 0.291 | 1.545 | 5.32× |
| 100 | 10 | 0.617 | 3.229 | 5.23× |
| 1,000 | 1 | 0.103 | 1.189 | 11.53× |
| 1,000 | 5 | 0.289 | 5.213 | 18.02× |
| 1,000 | 10 | 0.618 | 11.433 | 18.49× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
