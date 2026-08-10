# UlcerIndex benchmark (`UlcerIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 48.50M | 0.020 | 49.12M | 0.181 | 8.76× | 8.87× |
| 10,000 | 0.232 | 43.10M | 0.193 | 51.80M | 0.582 | 2.51× | 3.02× |
| 100,000 | 1.974 | 50.66M | 1.859 | 53.79M | 4.552 | 2.31× | 2.45× |
| 1,000,000 | 18.941 | 52.79M | 18.667 | 53.57M | 45.924 | 2.42× | 2.46× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.064 | 0.266 | 4.18× |
| 1 | 5 | 0.286 | 1.493 | 5.23× |
| 1 | 10 | 0.501 | 2.399 | 4.79× |
| 10 | 1 | 0.048 | 0.207 | 4.36× |
| 10 | 5 | 0.244 | 1.423 | 5.83× |
| 10 | 10 | 0.493 | 2.342 | 4.75× |
| 100 | 1 | 0.053 | 0.225 | 4.25× |
| 100 | 5 | 0.247 | 1.366 | 5.53× |
| 100 | 10 | 0.502 | 2.364 | 4.71× |
| 1,000 | 1 | 0.069 | 0.280 | 4.04× |
| 1,000 | 5 | 0.245 | 1.674 | 6.84× |
| 1,000 | 10 | 0.508 | 2.841 | 5.60× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
