# RollingSharpe benchmark (`SharpeRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.389 | 2.57M | 0.387 | 2.58M | 0.187 | 0.48× | 0.48× |
| 10,000 | 4.136 | 2.42M | 4.454 | 2.25M | 0.552 | 0.13× | 0.12× |
| 100,000 | 39.237 | 2.55M | 41.189 | 2.43M | 4.010 | 0.10× | 0.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.150 | 0.234 | 1.56× |
| 1 | 5 | 0.433 | 1.344 | 3.11× |
| 1 | 10 | 0.600 | 2.348 | 3.91× |
| 10 | 1 | 0.072 | 0.221 | 3.07× |
| 10 | 5 | 0.293 | 1.255 | 4.28× |
| 10 | 10 | 0.584 | 2.303 | 3.95× |
| 100 | 1 | 0.104 | 0.224 | 2.15× |
| 100 | 5 | 0.306 | 1.271 | 4.16× |
| 100 | 10 | 0.640 | 2.378 | 3.72× |
| 1,000 | 1 | 0.468 | 0.262 | 0.56× |
| 1,000 | 5 | 0.709 | 1.494 | 2.11× |
| 1,000 | 10 | 1.154 | 2.734 | 2.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
