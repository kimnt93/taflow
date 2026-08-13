# EaseOfMovement benchmark (`EaseOfMovement` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.073 | 13.65M | 0.065 | 15.31M | 0.283 | 3.87× | 4.34× |
| 10,000 | 0.564 | 17.75M | 0.553 | 18.09M | 1.270 | 2.25× | 2.30× |
| 100,000 | 5.211 | 19.19M | 5.308 | 18.84M | 10.914 | 2.09× | 2.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.129 | 0.291 | 2.26× |
| 1 | 5 | 0.384 | 1.508 | 3.93× |
| 1 | 10 | 0.718 | 2.832 | 3.94× |
| 10 | 1 | 0.088 | 0.264 | 3.01× |
| 10 | 5 | 0.322 | 1.510 | 4.69× |
| 10 | 10 | 0.662 | 2.619 | 3.95× |
| 100 | 1 | 0.086 | 0.258 | 2.98× |
| 100 | 5 | 0.322 | 1.578 | 4.90× |
| 100 | 10 | 0.734 | 3.209 | 4.37× |
| 1,000 | 1 | 0.145 | 0.376 | 2.60× |
| 1,000 | 5 | 0.336 | 2.151 | 6.40× |
| 1,000 | 10 | 0.705 | 3.776 | 5.35× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
