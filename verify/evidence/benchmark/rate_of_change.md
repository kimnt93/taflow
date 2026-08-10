# RateOfChange benchmark (`ROC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 180.79M | 0.005 | 200.54M | 0.052 | 9.43× | 10.46× |
| 10,000 | 0.023 | 441.77M | 0.019 | 516.11M | 0.044 | 1.96× | 2.29× |
| 100,000 | 0.202 | 494.13M | 0.180 | 556.61M | 0.184 | 0.91× | 1.02× |
| 1,000,000 | 4.561 | 219.23M | 2.410 | 414.97M | 1.544 | 0.34× | 0.64× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.063 | 0.115 | 1.82× |
| 1 | 5 | 0.254 | 0.505 | 1.99× |
| 1 | 10 | 0.500 | 1.009 | 2.02× |
| 10 | 1 | 0.047 | 0.087 | 1.85× |
| 10 | 5 | 0.272 | 0.479 | 1.76× |
| 10 | 10 | 0.489 | 0.962 | 1.97× |
| 100 | 1 | 0.049 | 0.089 | 1.81× |
| 100 | 5 | 0.254 | 0.489 | 1.93× |
| 100 | 10 | 0.529 | 1.058 | 2.00× |
| 1,000 | 1 | 0.053 | 0.097 | 1.83× |
| 1,000 | 5 | 0.268 | 0.472 | 1.76× |
| 1,000 | 10 | 0.605 | 1.134 | 1.87× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
