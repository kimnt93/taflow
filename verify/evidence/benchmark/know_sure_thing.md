# KnowSureThing benchmark (`KST` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 48.41M | 0.019 | 51.44M | 0.706 | 34.17× | 36.30× |
| 10,000 | 0.163 | 61.18M | 0.159 | 62.81M | 3.503 | 21.43× | 22.00× |
| 100,000 | 1.624 | 61.57M | 1.533 | 65.25M | 35.167 | 21.65× | 22.95× |
| 1,000,000 | 17.487 | 57.19M | 15.751 | 63.49M | 398.007 | 22.76× | 25.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.113 | 0.601 | 5.31× |
| 1 | 5 | 0.270 | 2.594 | 9.60× |
| 1 | 10 | 0.503 | 5.195 | 10.32× |
| 10 | 1 | 0.056 | 0.488 | 8.70× |
| 10 | 5 | 0.247 | 2.665 | 10.80× |
| 10 | 10 | 0.488 | 5.240 | 10.74× |
| 100 | 1 | 0.053 | 0.559 | 10.58× |
| 100 | 5 | 0.259 | 2.832 | 10.94× |
| 100 | 10 | 0.523 | 5.494 | 10.50× |
| 1,000 | 1 | 0.082 | 1.005 | 12.19× |
| 1,000 | 5 | 0.246 | 4.592 | 18.65× |
| 1,000 | 10 | 0.520 | 9.509 | 18.29× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
