# Liquidity benchmark (`causal liquidity pools` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.217 | 4.62M | 0.200 | 5.01M | 4.396 | 20.30× | 22.01× |
| 10,000 | 2.147 | 4.66M | 2.143 | 4.67M | 62.436 | 29.08× | 29.13× |
| 100,000 | 25.546 | 3.91M | 24.756 | 4.04M | 1085.936 | 42.51× | 43.87× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.223 | 0.256 | 1.15× |
| 1 | 5 | 0.407 | 0.563 | 1.39× |
| 1 | 10 | 0.681 | 1.125 | 1.65× |
| 10 | 1 | 0.071 | 0.123 | 1.72× |
| 10 | 5 | 0.324 | 0.582 | 1.79× |
| 10 | 10 | 0.701 | 1.235 | 1.76× |
| 100 | 1 | 0.092 | 0.203 | 2.22× |
| 100 | 5 | 0.322 | 0.981 | 3.05× |
| 100 | 10 | 0.714 | 1.962 | 2.75× |
| 1,000 | 1 | 0.286 | 4.797 | 16.80× |
| 1,000 | 5 | 0.587 | 26.108 | 44.49× |
| 1,000 | 10 | 1.193 | 102.219 | 85.68× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
