# RollingMaximumIndex benchmark (`MAXINDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 129.73M | 0.007 | 146.18M | 0.036 | 4.73× | 5.33× |
| 10,000 | 0.055 | 182.80M | 0.053 | 187.41M | 0.094 | 1.71× | 1.75× |
| 100,000 | 0.518 | 193.20M | 0.482 | 207.67M | 0.656 | 1.27× | 1.36× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.147 | 0.132 | 0.90× |
| 1 | 5 | 0.400 | 0.525 | 1.31× |
| 1 | 10 | 0.482 | 1.027 | 2.13× |
| 10 | 1 | 0.060 | 0.092 | 1.53× |
| 10 | 5 | 0.241 | 0.443 | 1.84× |
| 10 | 10 | 0.486 | 0.917 | 1.89× |
| 100 | 1 | 0.047 | 0.096 | 2.02× |
| 100 | 5 | 0.226 | 0.422 | 1.87× |
| 100 | 10 | 0.487 | 0.925 | 1.90× |
| 1,000 | 1 | 0.055 | 0.099 | 1.81× |
| 1,000 | 5 | 0.236 | 0.464 | 1.97× |
| 1,000 | 10 | 0.502 | 0.998 | 1.99× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
