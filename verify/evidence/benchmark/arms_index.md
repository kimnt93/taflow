# ArmsIndex benchmark (`Trin` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 86.41M | 0.009 | 111.09M | 9.097 | 786.05× | 1010.55× |
| 10,000 | 0.045 | 220.53M | 0.040 | 248.18M | 87.117 | 1921.14× | 2162.05× |
| 100,000 | 0.351 | 285.15M | 0.321 | 311.27M | 897.670 | 2559.72× | 2794.16× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.135 | 0.351 | 2.59× |
| 1 | 5 | 0.317 | 1.258 | 3.97× |
| 1 | 10 | 0.544 | 2.580 | 4.74× |
| 10 | 1 | 0.054 | 0.315 | 5.82× |
| 10 | 5 | 0.380 | 1.626 | 4.28× |
| 10 | 10 | 0.561 | 3.420 | 6.09× |
| 100 | 1 | 0.057 | 1.154 | 20.32× |
| 100 | 5 | 0.291 | 5.958 | 20.51× |
| 100 | 10 | 0.565 | 11.981 | 21.22× |
| 1,000 | 1 | 0.073 | 9.344 | 128.14× |
| 1,000 | 5 | 0.365 | 46.468 | 127.35× |
| 1,000 | 10 | 0.665 | 97.312 | 146.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
