# 实现的功能

为 TCB 增加以下成员：
- `begin` 表示任务是否已经开始运行
- `st_time` 表示任务初次开始运行的时间
- `syscall_counter` 是系统调用的计数器

在调度任务时判断是否为初次调度，如果是则更新 `st_time`。

调用系统调用时先更新 `syscall_counter`。

调用`sys_task_info` 时则计算当前时间和 `st_time`的时间差，以及直接复制 `syscall_counter`

# 问答题

1.

程序出错行为：
```
[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
```
1中访问了地址为0x0的内存，导致PageFault
2中在U态使用`sret`指令，3的指令访问`sstatus`寄存器导致非法指令
rustsbi(版本为拉取下来的版本)

2-1.

刚进入`__restore`时，`a0`是`trap_handler`函数中`cx`的地址, `cx`存储用户态的上下文

`__restore`的使用场景一个是刚开始时直接从这里运行，以进入U态执行用户任务。另一个是U态陷入S态，并从`trap_handler`返回后，返回这里以返回U态继续用户任务。

2-2.

`32*8(sp)`是异常发生时的`sstatus`寄存器，`33*8(sp)`是异常发生时的pc，`2*8(sp)`是临时保存的用户栈指针，这些更改以便恢复异常之前的`sstatus`状态、用户pc和用户栈指针，然后准备返回U态

2-3.

`x2`和`x4`是栈指针和线程指针，一个临时保存在`sscratch`中了，一个不使用

2-4.

交换了内核栈指针和用户栈指针，`__restore`中该指令后`sp`就是用户栈指针，`sscratch`就是内核栈指针

2-5.

`sret`指令，`sret`从`sepc`中恢复pc，cpu切换特权级别。

2-6.

和问题4一样，交换了内核栈指针和用户栈指针，但是`__alltraps`中该指令后`sp`就是内核栈指针，`sscratch`就是用户栈指针

2-7.

`ecall`指令、导致异常的指令（比如访存错误、指令格式错误）、时钟中断产生时任意指令均会进入S态

# 荣誉准则

    在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

        没有，没有，没有。

    此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

        [实验指导书](https://learningos.cn/rCore-Camp-Guide-2024A/chapter3/5exercise.html#id1)

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

