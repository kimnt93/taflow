# AccumulationDistribution benchmark (`AD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 66.31M | 0.009 | 109.65M | 0.032 | 2.15× | 3.56× |
| 10,000 | 0.044 | 227.22M | 0.040 | 248.98M | 0.046 | 1.04× | 1.14× |
| 100,000 | 0.374 | 267.48M | 0.329 | 304.24M | 0.195 | 0.52× | 0.59× |
| 1,000,000 | 5.044 | 198.26M | 4.163 | 240.23M | 2.321 | 0.46× | 0.56× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.104 | 0.108 | 1.04× |
| 1 | 5 | 0.271 | 0.542 | 2.00× |
| 1 | 10 | 0.561 | 0.959 | 1.71× |
| 10 | 1 | 0.056 | 0.088 | 1.56× |
| 10 | 5 | 0.273 | 0.531 | 1.94× |
| 10 | 10 | 0.582 | 0.972 | 1.67× |
| 100 | 1 | 0.050 | 0.085 | 1.72× |
| 100 | 5 | 0.262 | 0.486 | 1.85× |
| 100 | 10 | 0.604 | 1.010 | 1.67× |
| 1,000 | 1 | 0.055 | 0.096 | 1.76× |
| 1,000 | 5 | 0.266 | 0.495 | 1.86× |
| 1,000 | 10 | 0.646 | 1.151 | 1.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
