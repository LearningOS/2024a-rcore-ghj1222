# 实现的功能

实现了spawn，具体来说和fork的过程差不多，但是部分内容不继承父进程，也不复制父进程的地址空间。同时使用了execve的解析elf部分，按照execve的方式初始地址空间。

实现stride调度，在进程控制块中增加两个字段stride和priority，在每次调度到进程时增加一个stride。本来想使用二叉堆管理进程，但是不知道为什么会卡住，所以使用一个数组管理进程，每次找到stride最小的进程。

# 问答题

1. 实际不是p1执行，因为p2增加后发生溢出了，如果不检测溢出应该是变为4,还是p2执行

2. 因为每次增加的pass=BigStride/prio，因为prio>=2，所以某个程序增加的pass<=BigStride/2，而每次只调度stride最小的程序，如果一开始就满足STRIDE_MAX-STRIDE_MIN<=BigStride/2，调度最小程序之后的STRIDE_MIN'>=STRIDE_MIN, STRIDE_MAX'=MAX(STRIDE_MAX, STRIDE_MIN+BigStride/2)，STRIDE_MAX'-STRIDE_MIN'<=MAX(STRIDE_MAX, STRIDE_MIN+BigStride/2)-STRIDE_MIN<=BigStride/2


3.
假设`BigStride/2<=u64::MAX/2`即可。

```rust
fn partical_cmp(&self, other: &Self) -> Option<Ordering> {
    
    let half = (std::u64::MAX/2)+1;
    if self.0 > other.0 {
        if (self.0-other.0) >= half {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    } else {
        if (other.0-self.0) >= half {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

```

# 荣誉准则

    在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

        没有，没有，没有。

    此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

        [实验指导书](https://learningos.cn/rCore-Camp-Guide-2024A/chapter5/4exercise.html)

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
