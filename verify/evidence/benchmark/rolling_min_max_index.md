# RollingMinMaxIndex benchmark (`MINMAXINDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 46.70M | 0.019 | 52.00M | 0.045 | 2.08× | 2.32× |
| 10,000 | 0.306 | 32.73M | 0.264 | 37.83M | 0.152 | 0.50× | 0.58× |
| 100,000 | 2.828 | 35.36M | 2.745 | 36.43M | 1.194 | 0.42× | 0.43× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.138 | 1.89× |
| 1 | 5 | 0.224 | 0.507 | 2.27× |
| 1 | 10 | 0.393 | 1.019 | 2.59× |
| 10 | 1 | 0.044 | 0.098 | 2.23× |
| 10 | 5 | 0.190 | 0.458 | 2.41× |
| 10 | 10 | 0.393 | 0.961 | 2.44× |
| 100 | 1 | 0.046 | 0.100 | 2.18× |
| 100 | 5 | 0.223 | 0.523 | 2.34× |
| 100 | 10 | 0.434 | 0.953 | 2.19× |
| 1,000 | 1 | 0.070 | 0.105 | 1.50× |
| 1,000 | 5 | 0.201 | 0.515 | 2.55× |
| 1,000 | 10 | 0.469 | 1.108 | 2.36× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
