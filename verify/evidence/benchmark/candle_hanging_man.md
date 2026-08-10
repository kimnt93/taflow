# CandleHangingMan benchmark (`CDLHANGINGMAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.93M | 0.017 | 58.30M | 0.043 | 2.05× | 2.49× |
| 10,000 | 0.183 | 54.63M | 0.176 | 56.85M | 0.179 | 0.98× | 1.02× |
| 100,000 | 1.757 | 56.92M | 1.734 | 57.69M | 1.712 | 0.97× | 0.99× |
| 1,000,000 | 19.432 | 51.46M | 18.511 | 54.02M | 14.820 | 0.76× | 0.80× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.154 | 1.71× |
| 1 | 5 | 0.326 | 0.471 | 1.44× |
| 1 | 10 | 0.520 | 0.956 | 1.84× |
| 10 | 1 | 0.058 | 0.114 | 1.95× |
| 10 | 5 | 0.293 | 0.480 | 1.64× |
| 10 | 10 | 0.574 | 0.908 | 1.58× |
| 100 | 1 | 0.061 | 0.089 | 1.46× |
| 100 | 5 | 0.281 | 0.462 | 1.64× |
| 100 | 10 | 0.670 | 0.987 | 1.47× |
| 1,000 | 1 | 0.077 | 0.101 | 1.31× |
| 1,000 | 5 | 0.266 | 0.501 | 1.88× |
| 1,000 | 10 | 0.665 | 1.165 | 1.75× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
