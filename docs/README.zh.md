
<p align=center>
  <br>
  <a href="https://sherlock-project.github.io/" target="_blank"><img src="docs/images/sherlock-logo.png"/></a>
  <br>
  <span>通过用户名在 <a href="https://sherlockproject.xyz/sites">400多个社交网络</a> 上查找社交媒体账户</span>
  <br>
</p>

<p align="center">
<img width="70%" height="70%" src="docs/images/demo.png"/>
</a>
</p>


## 安装

```shell
cargo install sherlock
```

## 使用

```shell
sherlock user123
```
要搜索多个用户：
```shell
sherlock user1 user2 user3
```
找到的账户将保存在一个单独的文本文件中，文件名为相应的用户名（例如 user123.txt）。

```shell
$ sherlock --help

通过用户名查找社交媒体账户

用法：sherlock-rs [OPTIONS] <usernames>...

参数：
  <usernames>...  一个或多个要与社交网络检查的用户名。使用 {?} 检查相似用户名（替换为'_', '-', '.'）

选项：
  -v, --verbose                        显示额外的调试信息和指标
  -o, --output <OUTPUT_FILE>           保存结果的输出文件
  -f, --output-folder <OUTPUT_FOLDER>  如果使用单个用户名，结果将保存到此文件夹
  -c, --csv                            创建逗号分隔值 (CSV) 文件
      --xlsx                           创建现代 Microsoft Excel 电子表格的标准文件 (xlsx)
  -s, --site-list <SITE_LIST>          限制分析仅限于列出的站点。添加多个选项以指定多个站点
  -p, --proxy <PROXY>
  -d, --dump-response                  将 HTTP 请求转储到 stdout 以进行有针对性的调试
  -j, --json <JSON_FILE>               从 JSON 文件或在线有效 JSON 文件加载数据
  -t, --timeout <TIMEOUT>              等待请求响应的时间（秒）[默认值：60]
      --print-all                      输出未找到用户名的站点
      --print-found                    输出找到用户名的站点
  -n, --no-color                       不为终端输出着色
  -b, --browse                         在默认浏览器中浏览所有结果
  -l, --local                          强制使用本地 data.json 文件
      --nsfw                           包括检查默认列表中的 NSFW 站点
  -h, --help                           打印帮助
  -V, --version                        打印版本
```

## 安装

```shell
cargo install sherlock
```

## 动机

我非常不喜欢部署 Python 应用程序，并且想要一个单独的二进制文件。Go 让我很恼火，Swift 在服务器端和 Linux 上没有很好的工具，C++ 甚至不在考虑范围内，所以选择了 Rust。

<p align="center">
<img width="70%" height="70%" src="docs/images/reddit.png"/>
</a>
</p>

## 许可证

MIT © Johannes Naylor<br/>
Sherlock 的原始创建者 - [Siddharth Dushantha](https://github.com/sdushantha)

## 鸣谢

- [Siddharth Dushantha](https://github.com/sdushantha) 创建了原始的 Sherlock
- [Eira Fransham](https://github.com/eira-fransham) 提供了 crate 名称
