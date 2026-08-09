# MovingAverageConvergenceDivergenceFixed benchmark (`MACDFIX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 149.18M | 0.005 | 198.06M | 0.051 | 7.54× | 10.01× |
| 10,000 | 0.034 | 297.94M | 0.026 | 386.96M | 0.138 | 4.11× | 5.33× |
| 100,000 | 0.301 | 332.31M | 0.226 | 441.51M | 1.014 | 3.37× | 4.48× |
| 1,000,000 | 13.082 | 76.44M | 2.475 | 404.05M | 15.104 | 1.15× | 6.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.067 | 0.126 | 1.88× |
| 1 | 5 | 0.281 | 0.577 | 2.05× |
| 1 | 10 | 0.492 | 1.027 | 2.09× |
| 10 | 1 | 0.051 | 0.106 | 2.09× |
| 10 | 5 | 0.230 | 0.486 | 2.11× |
| 10 | 10 | 0.505 | 1.067 | 2.12× |
| 100 | 1 | 0.051 | 0.105 | 2.06× |
| 100 | 5 | 0.231 | 0.484 | 2.09× |
| 100 | 10 | 0.466 | 1.032 | 2.22× |
| 1,000 | 1 | 0.062 | 0.123 | 1.99× |
| 1,000 | 5 | 0.258 | 0.563 | 2.18× |
| 1,000 | 10 | 0.484 | 1.132 | 2.34× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | Reference full µs | vs full | vs bounded tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.416 | 0.283 | 3.53M | 1033.234 | 3646.68× | 146.12× |
| 100,000 | 10 | 1.935 | 1.832 | 5.46M | 1013.412 | 553.09× | 22.74× |
| 100,000 | 1,000 | 112.568 | 70.359 | 14.21M | 1030.067 | 14.64× | 0.74× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | Reference vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 101.62M | 242.80M | 1.00× | 1.65M | 1.76M | 1.00× | 74.07M |
| 5 | 175.88M | 491.38M | 2.02× | 1.44M | 1.20M | 0.68× | 78.45M |
| 10 | 188.47M | 522.00M | 2.15× | 1.49M | 1.35M | 0.77× | 80.64M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
