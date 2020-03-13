# Signal stack overflow examples
This repo contains two example programs that, in debug builds, overflow their signal stacks and demonstrate some possible outcomes in a rust program built today. These also serve as demonstrations for failure modes (`Segmentation fault (core dumped)`) in rusts built with the changes in [WIP].

`cargo run --bin sigstackoverflow` demonstrates a signal stack overflow in a signal handler handling `SIGSEGV`, where `cargo run --bin sigalrmy` demonstrates a signal stack overflow in a handler handling `SIGALRM`. Release builds don't cause errors because the padding array is found to be unneeded and discarded.

## Why? Signal stack overflows aren't great.
When a custom signal handler is run on a libstd-provided alternate signal stack, which could be the case when a program installs a signal handler intended for some, but not all, threads, that handler may overflow default `SIGSTKSZ` signal stacks. In the lucky case, an unmapped page, page with `PROT_NONE`, or otherwise invalid permissisons is immediately after and immediately causes either a `SIGSEGV` or `SIGBUS`. In the unlucky case, some other read/write memory exists below the alternate signal stack, and the overflow may go unnoticed until some unlucky scenario where the overflow clobbers important data.

A realistic example of this may be a program that installs a custom signal handler, where the intended execution of the handler is paired with custom larger-than-default signal stacks provisioned for the expected stack use. Then, a signal that arrives on an unexpected thread would cause the process-wide signal handler to run on an unexpectedly small signal stack, to any of the above consequences.
