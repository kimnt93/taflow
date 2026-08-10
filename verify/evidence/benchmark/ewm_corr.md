# ExponentiallyWeightedCorrelation benchmark (`ewm correlation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 105.00M | 0.008 | 118.25M | 1.180 | 123.95× | 139.59× |
| 10,000 | 0.062 | 160.93M | 0.055 | 182.56M | 12.289 | 197.76× | 224.34× |
| 100,000 | 0.551 | 181.64M | 0.529 | 189.15M | 118.046 | 214.42× | 223.28× |
| 1,000,000 | 5.532 | 180.76M | 5.056 | 197.79M | 1249.871 | 225.93× | 247.22× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.128 | 1.45× |
| 1 | 5 | 0.301 | 0.572 | 1.90× |
| 1 | 10 | 0.482 | 1.073 | 2.23× |
| 10 | 1 | 0.048 | 0.117 | 2.44× |
| 10 | 5 | 0.232 | 0.565 | 2.43× |
| 10 | 10 | 0.496 | 1.203 | 2.43× |
| 100 | 1 | 0.050 | 0.233 | 4.62× |
| 100 | 5 | 0.232 | 1.131 | 4.87× |
| 100 | 10 | 0.495 | 2.249 | 4.54× |
| 1,000 | 1 | 0.055 | 1.364 | 24.76× |
| 1,000 | 5 | 0.234 | 6.849 | 29.29× |
| 1,000 | 10 | 0.496 | 13.986 | 28.17× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
