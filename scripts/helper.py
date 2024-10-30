import math
from typing import List

import polars as pl


def calc_mean(df: pl.DataFrame, diff: str = 'diff', count: str = 'count') -> float:
    tmp = df.with_columns((pl.col(diff) * pl.col(count)).alias('mul')).select(pl.col('mul')).sum().item()
    tmp /= df.select(pl.col('count')).sum().item()
    return tmp


def calc_std_dev(df: pl.DataFrame, mean: float = None, diff: str = 'diff', count: str = 'count') -> float:
    if mean is None:
        mean = calc_mean(df, diff, count)

    tmp = df.with_columns(((mean - pl.col('diff')) ** 2).alias("var"))
    tmp = tmp.with_columns((pl.col('var') * pl.col('count')).alias('var')).select(pl.col('var')).sum().item() / (
        df['count'].sum())
    return math.sqrt(tmp)


def summary(df: pl.DataFrame, diff: str = 'diff', count: str = 'count') -> dict[str, float]:
    mean = calc_mean(df, diff, count)
    return {'mean': mean, 'std_dev': calc_std_dev(df, mean, diff, count)}


def calc_sample(rng: int) -> List[int]:
    if rng <= 0:
        raise ValueError("Invalid value")

    list = [rng]
    cnt = (rng - 1) * 2

    for i in range(rng - 1):
        list.append(cnt)
        cnt -= 2

    return list
