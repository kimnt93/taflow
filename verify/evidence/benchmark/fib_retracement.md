# FibonacciRetracement benchmark (`rolling Fibonacci levels` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.029 | 33.99M | 0.025 | 39.72M | 10.516 | 357.46× | 417.73× |
| 10,000 | 0.297 | 33.69M | 0.280 | 35.77M | 108.505 | 365.50× | 388.12× |
| 100,000 | 3.483 | 28.71M | 2.829 | 35.35M | 1045.768 | 300.23× | 369.70× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.124 | 0.163 | 1.32× |
| 1 | 5 | 0.393 | 0.574 | 1.46× |
| 1 | 10 | 0.478 | 1.262 | 2.64× |
| 10 | 1 | 0.052 | 0.214 | 4.12× |
| 10 | 5 | 0.232 | 1.083 | 4.67× |
| 10 | 10 | 0.656 | 2.643 | 4.03× |
| 100 | 1 | 0.060 | 1.197 | 19.85× |
| 100 | 5 | 0.258 | 6.085 | 23.60× |
| 100 | 10 | 0.513 | 12.374 | 24.11× |
| 1,000 | 1 | 0.097 | 11.304 | 116.10× |
| 1,000 | 5 | 0.317 | 59.156 | 186.53× |
| 1,000 | 10 | 0.696 | 128.701 | 184.80× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
