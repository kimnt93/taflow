# RollingMaximumIndex benchmark (`MAXINDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 148.84M | 0.005 | 189.23M | 0.037 | 5.48× | 6.97× |
| 10,000 | 0.053 | 188.14M | 0.050 | 201.38M | 0.096 | 1.81× | 1.94× |
| 100,000 | 0.525 | 190.33M | 0.491 | 203.46M | 0.718 | 1.37× | 1.46× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.052 | 0.119 | 2.29× |
| 1 | 5 | 0.274 | 0.453 | 1.66× |
| 1 | 10 | 0.379 | 0.957 | 2.52× |
| 10 | 1 | 0.048 | 0.096 | 1.99× |
| 10 | 5 | 0.183 | 0.435 | 2.38× |
| 10 | 10 | 0.376 | 0.933 | 2.48× |
| 100 | 1 | 0.043 | 0.104 | 2.43× |
| 100 | 5 | 0.236 | 0.488 | 2.07× |
| 100 | 10 | 0.385 | 0.889 | 2.31× |
| 1,000 | 1 | 0.053 | 0.103 | 1.93× |
| 1,000 | 5 | 0.191 | 0.453 | 2.37× |
| 1,000 | 10 | 0.445 | 1.009 | 2.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
