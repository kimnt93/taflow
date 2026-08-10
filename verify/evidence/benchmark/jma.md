# JurikMovingAverage benchmark (`jma` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.087 | 11.54M | 0.087 | 11.49M | 20.026 | 231.09× | 230.06× |
| 10,000 | 0.890 | 11.23M | 0.929 | 10.76M | 204.649 | 229.83× | 220.20× |
| 100,000 | 11.665 | 8.57M | 8.878 | 11.26M | 1937.219 | 166.07× | 218.20× |
| 1,000,000 | 88.834 | 11.26M | 87.867 | 11.38M | 19321.328 | 217.50× | 219.89× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.212 | 0.223 | 1.05× |
| 1 | 5 | 0.262 | 0.833 | 3.18× |
| 1 | 10 | 0.466 | 1.629 | 3.50× |
| 10 | 1 | 0.049 | 0.494 | 10.02× |
| 10 | 5 | 0.235 | 2.437 | 10.38× |
| 10 | 10 | 0.489 | 4.912 | 10.05× |
| 100 | 1 | 0.058 | 2.315 | 39.98× |
| 100 | 5 | 0.237 | 11.317 | 47.70× |
| 100 | 10 | 0.531 | 23.719 | 44.69× |
| 1,000 | 1 | 0.197 | 20.147 | 102.46× |
| 1,000 | 5 | 0.374 | 172.164 | 459.81× |
| 1,000 | 10 | 0.652 | 222.647 | 341.58× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
