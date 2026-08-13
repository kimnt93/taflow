# RollingPainIndex benchmark (`PainIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.212 | 4.71M | 0.243 | 4.12M | 0.163 | 0.77× | 0.67× |
| 10,000 | 2.063 | 4.85M | 2.071 | 4.83M | 0.629 | 0.30× | 0.30× |
| 100,000 | 21.041 | 4.75M | 20.323 | 4.92M | 5.412 | 0.26× | 0.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.174 | 0.211 | 1.21× |
| 1 | 5 | 0.460 | 0.984 | 2.14× |
| 1 | 10 | 0.609 | 2.094 | 3.44× |
| 10 | 1 | 0.071 | 0.184 | 2.59× |
| 10 | 5 | 0.283 | 0.926 | 3.27× |
| 10 | 10 | 0.595 | 2.084 | 3.50× |
| 100 | 1 | 0.088 | 0.192 | 2.17× |
| 100 | 5 | 0.290 | 0.938 | 3.23× |
| 100 | 10 | 0.607 | 2.186 | 3.60× |
| 1,000 | 1 | 0.304 | 0.284 | 0.93× |
| 1,000 | 5 | 0.459 | 1.208 | 2.63× |
| 1,000 | 10 | 0.852 | 2.643 | 3.10× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
