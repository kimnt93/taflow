# FibonacciArcs benchmark (`FibArcs` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 45.80M | 0.020 | 50.71M | 0.501 | 22.95× | 25.42× |
| 10,000 | 0.168 | 59.52M | 0.158 | 63.29M | 4.011 | 23.87× | 25.38× |
| 100,000 | 1.691 | 59.13M | 1.534 | 65.18M | 44.574 | 26.36× | 29.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.247 | 2.46× |
| 1 | 5 | 0.284 | 0.847 | 2.99× |
| 1 | 10 | 0.483 | 1.904 | 3.94× |
| 10 | 1 | 0.067 | 0.203 | 3.00× |
| 10 | 5 | 0.256 | 0.892 | 3.48× |
| 10 | 10 | 0.513 | 2.125 | 4.14× |
| 100 | 1 | 0.065 | 0.246 | 3.76× |
| 100 | 5 | 0.269 | 1.073 | 3.99× |
| 100 | 10 | 0.517 | 2.483 | 4.81× |
| 1,000 | 1 | 0.069 | 0.770 | 11.13× |
| 1,000 | 5 | 0.252 | 3.283 | 13.02× |
| 1,000 | 10 | 0.535 | 6.422 | 12.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
