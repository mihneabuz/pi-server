# Linux zero-copy IO

Performing IO operations requires that you *somehow* go through the kernel, because that
is what manages all resources and permissions on a system. Typically, this traversal
occurs via a system call, and in Linux, the most ubiquitous IO system calls are (the beautifully named)
[read](https://man7.org/linux/man-pages/man2/read.2.html) and [write](https://man7.org/linux/man-pages/man2/write.2.html).

Let's take a look at how they work.
```c
ssize_t read(int fd, void buf, size_t count);

ssize_t write(int fd, const void buf, size_t count);
```
Both take in a file descriptor, a pointer to a buffer, the size of that buffer and return how many bytes
were read/written. The file descriptor is simply an id of the *resource* that is being operated on.
This can represent a file or a network socket or any other entity that supports these operations.


## Blocking

An important thing to mention here is that these are potentially blocking calls. If the resource being
operated on is not able to process this request immediately, the kernel will force the calling thread to
yield.

This is because the resource could take a *long* time to become available. Even though from our perspective
these operations feel instant, compared to the speed of the CPU, fetching data from a hard drive *feels like years*.


## Copying

One of the most common operations of a computer is to copy data from one place to another.
Just think of all http servers that host static files - just like this website.

Doing this using what we learned to far could look like this:

```c
#include "unistd.h"

#define BUF_SIZE 1024

int copy(int from_fd, int to_fd) {
    char buf[BUF_SIZE];
    int rd, wr, total;

    while (1) {
        rd = read(from_fd, buf, sizeof(buf));
        if (rd < 0)
            return rd;

        if (rd == 0)
            break; // we're done!

        wr = write(to_fd, buf, rd < sizeof(buf) ? rd : sizeof(buf));
        if (wr < 0)
            return wr;

        total += wr;
    }

    return total;
}
```

We keep a temporary buffer, and do a loop. On each iteration we try to read from the source into `buf`
and then write from `buf` to the destination. When there is nothing to read anymore we break.

This works ok, and you can tweak the buffer size to achieve optimal performance, but it just feels a
bit inefficient. We have to do a system call to copy the data from the kernel to our buffer, just so
we can then copy it back to the kernel immediately. We also have to keep in mind that system calls have
their own cost, not to mention the constant blocking our poor thread would endure.

Wouldn't it be a lot nicer if we could just tell the kernel to do both these operations at once?
Turns out we can!

## Sendfile

[Sendfile](https://man7.org/linux/man-pages/man2/sendfile.2.html) is a system call that does just that, sends a file over a socket. Well technically, the destination
can be any file descriptor (just like out read-write loop), but the source *has* to be a regular file.

It looks like this:
```c
ssize_t sendfile(int out_fd, int in_fd, off_t *offset, size_t count);
```
The API is very similar, we pass the destination file descriptor, the source file descriptor of the file,
and, instead of a buffer, we specify the offset in the source file that we want to start sending from.

Let's use it to rewrite our read-write loop.
```c
#include <sys/sendfile.h>

#define CHUNK_SIZE 1024

int copy2(int from_fd, int to_fd) {
    off_t offset = 0;
    int ret, total;

    while (1) {
        // Note: the offset is automatically advanced by the call,
        //       that is why we have to pass in a pointer
        ret = sendfile(to_fd, from_fd, &offset, CHUNK_SIZE);
        if (ret < 0)
          return ret;

        if (ret == 0)
          break;

        total += ret;
    }

    return total;
}
```

Notice we still need to use a loop as we can't be certain that we can send the whole file with a single call.
Also, we are using a `CHUNK_SIZE` as a way to limit the maximum amount of bytes that should be copied with
each call, but we could just set it to the maximum allowed value of `0x7ffff000` and let the kernel deal with it.


I think this looks a lot nicer!


## Zero-copy

This technique is called zero-copy IO, we are performing IO operations without ever having the data in userspace,
without ever *copying* it between userspace and the kernel.

Why do we do it? Because it's more *efficient*, not necessarily *faster*. Again, the speed of the processor is so much
faster than the speed of any storage device or network card, so copying data to and from a buffer will never be the
bottleneck here. But when you have thousands of concurrent requests it will reduce the load on the CPU.

> There is also the [splice](https://man7.org/linux/man-pages/man2/splice.2.html) system call that can copy between an arbitrary file descriptor and a pipe
