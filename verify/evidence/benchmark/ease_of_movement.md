# EaseOfMovement benchmark (`EaseOfMovement` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 94.80M | 0.008 | 127.96M | 0.287 | 27.16× | 36.66× |
| 10,000 | 0.074 | 134.40M | 0.066 | 151.39M | 1.253 | 16.84× | 18.96× |
| 100,000 | 0.677 | 147.69M | 0.647 | 154.49M | 10.741 | 15.86× | 16.59× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.355 | 4.83× |
| 1 | 5 | 0.308 | 1.569 | 5.09× |
| 1 | 10 | 0.397 | 2.642 | 6.65× |
| 10 | 1 | 0.049 | 0.313 | 6.34× |
| 10 | 5 | 0.187 | 1.505 | 8.03× |
| 10 | 10 | 0.406 | 2.938 | 7.23× |
| 100 | 1 | 0.044 | 0.256 | 5.78× |
| 100 | 5 | 0.194 | 1.577 | 8.12× |
| 100 | 10 | 0.454 | 2.870 | 6.32× |
| 1,000 | 1 | 0.055 | 0.350 | 6.37× |
| 1,000 | 5 | 0.202 | 2.177 | 10.79× |
| 1,000 | 10 | 0.451 | 4.269 | 9.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
