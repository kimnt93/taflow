# RateOfChangeRatio benchmark (`ROCR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 295.36M | 0.002 | 426.37M | 0.034 | 9.97× | 14.39× |
| 10,000 | 0.019 | 530.91M | 0.018 | 562.41M | 0.045 | 2.41× | 2.56× |
| 100,000 | 0.188 | 531.52M | 0.158 | 631.30M | 0.131 | 0.69× | 0.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.123 | 1.30× |
| 1 | 5 | 0.206 | 0.522 | 2.53× |
| 1 | 10 | 0.406 | 1.000 | 2.46× |
| 10 | 1 | 0.043 | 0.092 | 2.16× |
| 10 | 5 | 0.193 | 0.482 | 2.49× |
| 10 | 10 | 0.456 | 1.082 | 2.37× |
| 100 | 1 | 0.047 | 0.094 | 2.00× |
| 100 | 5 | 0.193 | 0.462 | 2.40× |
| 100 | 10 | 0.404 | 1.038 | 2.57× |
| 1,000 | 1 | 0.045 | 0.092 | 2.04× |
| 1,000 | 5 | 0.239 | 0.532 | 2.22× |
| 1,000 | 10 | 0.411 | 0.994 | 2.42× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
