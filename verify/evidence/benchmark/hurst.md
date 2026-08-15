# Hurst benchmark (`HurstExponent` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.357 | 2.80M | 0.353 | 2.83M | 0.486 | 1.36× | 1.38× |
| 10,000 | 3.549 | 2.82M | 3.691 | 2.71M | 3.485 | 0.98× | 0.94× |
| 100,000 | 35.386 | 2.83M | 35.689 | 2.80M | 33.071 | 0.93× | 0.93× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.335 | 4.06× |
| 1 | 5 | 0.317 | 1.623 | 5.13× |
| 1 | 10 | 0.418 | 2.926 | 7.00× |
| 10 | 1 | 0.055 | 0.263 | 4.76× |
| 10 | 5 | 0.206 | 1.484 | 7.21× |
| 10 | 10 | 0.404 | 2.619 | 6.49× |
| 100 | 1 | 0.078 | 0.286 | 3.68× |
| 100 | 5 | 0.208 | 1.638 | 7.87× |
| 100 | 10 | 0.460 | 3.159 | 6.86× |
| 1,000 | 1 | 0.427 | 0.588 | 1.38× |
| 1,000 | 5 | 0.614 | 3.235 | 5.27× |
| 1,000 | 10 | 0.864 | 6.175 | 7.15× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
