# ClearFreeBranchInLocal

## 命令

**删除已经合并的不存在远程本地分支：**

``` bash
git branch -vv | grep 'origin/.*: gone]' | awk '{print $1}' | xargs git branch -d
```
**删除未合并的不存在远程的本地分支：**

``` bash
git branch -vv | grep 'origin/.*: gone]' | awk '{print $1}' | xargs git branch -D
```

## 说明

- `git branch -vv`：列出所有本地分支及其追踪的远程分支。
- `grep 'origin/.*: gone]'`：过滤出追踪的远程分支已经不存在的本地分支。
- `awk '{print $1}'`：获取每行的第一个字段，即本地分支名。
- `xargs git branch -d`：将每个本地分支名作为参数，执行git branch -d命令删除分支

> 请注意，这个命令只会删除已经合并的本地分支。如果想要删除未合并的本地分支，可以将 `git branch -d` 替换为 `git branch -D`