# Parkinson benchmark (`ParkinsonVolatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.087 | 11.50M | 0.075 | 13.29M | 0.221 | 2.54× | 2.94× |
| 10,000 | 0.697 | 14.35M | 0.677 | 14.77M | 0.852 | 1.22× | 1.26× |
| 100,000 | 6.946 | 14.40M | 7.138 | 14.01M | 7.255 | 1.04× | 1.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.143 | 0.297 | 2.07× |
| 1 | 5 | 0.486 | 1.432 | 2.95× |
| 1 | 10 | 0.622 | 2.603 | 4.18× |
| 10 | 1 | 0.079 | 0.231 | 2.94× |
| 10 | 5 | 0.299 | 1.437 | 4.80× |
| 10 | 10 | 0.627 | 2.456 | 3.92× |
| 100 | 1 | 0.086 | 0.242 | 2.81× |
| 100 | 5 | 0.328 | 1.437 | 4.38× |
| 100 | 10 | 0.641 | 2.788 | 4.35× |
| 1,000 | 1 | 0.147 | 0.313 | 2.13× |
| 1,000 | 5 | 0.302 | 1.810 | 5.99× |
| 1,000 | 10 | 0.674 | 3.289 | 4.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
