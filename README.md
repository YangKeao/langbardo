# Langbardo

It's a library for quantile regression.

## Algorithm

The quantile regression was found by a linear constraint solver. With a list of
points `xs: &[f64]` and `ys: &[f64]`, the constraints are:

```
K * x_i + B_pos_i - B_neg_i = y_i

B_pos_i >= 0
B_neg_i <= 0
```

And try to minify `B_pos * quantile + B_neg * (1-quantile)`. Then we can get
a `K` and `B`, they are quantile regression for these points.

